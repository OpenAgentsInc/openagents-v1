use axum::{
    extract::{Query, Extension},
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::Duration;

use crate::server::services::{auth::{OIDCConfig, AuthResponse}, session::Session};

const SESSION_COOKIE_NAME: &str = "session";
const SESSION_DURATION_DAYS: i64 = 7;

#[derive(Debug, Deserialize)]
pub struct CallbackParams {
    code: String,
    // state: String, // TODO: Validate state parameter
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub config: OIDCConfig,
    pub pool: PgPool,
}

impl AppState {
    pub fn new(config: OIDCConfig, pool: PgPool) -> Self {
        Self { config, pool }
    }
}

pub async fn login(
    Extension(state): Extension<AppState>,
) -> impl IntoResponse {
    let auth_url = state.config.authorization_url();
    Json(LoginResponse { url: auth_url })
}

pub async fn callback(
    Extension(state): Extension<AppState>,
    Query(params): Query<CallbackParams>,
) -> Result<(HeaderMap, Json<AuthResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Exchange code for tokens and create session
    let auth_response = state.config.authenticate(params.code, &state.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    error: e.to_string(),
                })
            )
        })?;

    // Create session cookie
    let cookie = Cookie::build((SESSION_COOKIE_NAME, auth_response.session_token.clone()))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::days(SESSION_DURATION_DAYS))
        .build();

    // Return success response with cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok((headers, Json(auth_response)))
}

pub async fn logout(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
) -> (StatusCode, HeaderMap) {
    // Get session token from cookie
    let cookies = axum_extra::extract::cookie::CookieJar::from_headers(&headers);
    
    if let Some(cookie) = cookies.get(SESSION_COOKIE_NAME) {
        // Try to find and delete session
        if let Ok(session) = Session::validate(cookie.value(), &state.pool).await {
            let _ = session.delete(&state.pool).await;
        }
    }

    // Remove session cookie
    let removal_cookie = Cookie::build((SESSION_COOKIE_NAME, ""))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(Duration::seconds(0))
        .build();

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, removal_cookie.to_string().parse().unwrap());

    (StatusCode::OK, headers)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::services::test_helpers::{get_test_pool, cleanup_test_data, setup_test_db};
    use axum::{
        body::Body,
        http::Request,
        Router,
        routing::{get, post},
    };
    use tower::ServiceExt;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use lazy_static::lazy_static;
    use tokio::sync::Mutex;

    lazy_static! {
        static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
    }

    #[tokio::test]
    async fn test_auth_flow() {
        let _lock = TEST_MUTEX.lock().await;
        
        // Setup test database
        let pool = get_test_pool().await;
        cleanup_test_data(pool).await;
        setup_test_db(pool).await;

        // Setup mock OIDC server
        let mock_server = MockServer::start().await;

        // Create test config
        let config = OIDCConfig::new(
            "client123".to_string(),
            "secret456".to_string(),
            "http://localhost:3000/callback".to_string(),
            format!("{}/authorize", mock_server.uri()),
            format!("{}/token", mock_server.uri()),
            format!("{}/jwks", mock_server.uri()),
        )
        .unwrap();

        // Create a test JWT token signed with the same secret
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS256),
            &crate::server::services::auth::Claims {
                sub: "test_pseudonym".to_string(),
                exp: 1999999999,
                iat: 1516239022,
                iss: "https://auth.scramble.com".to_string(),
                aud: "client123".to_string(),
            },
            &jsonwebtoken::EncodingKey::from_secret(config.client_secret().as_bytes()),
        )
        .unwrap();

        // Setup mock responses
        Mock::given(method("POST"))
            .and(path("/token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "access_token": "test_access_token",
                "token_type": "Bearer",
                "expires_in": 3600,
                "id_token": token
            })))
            .mount(&mock_server)
            .await;

        // Create app state
        let state = AppState::new(config, pool.clone());

        // Create test app
        let app = Router::new()
            .route("/login", get(login))
            .route("/callback", get(callback))
            .route("/logout", post(logout))
            .layer(Extension(state));

        // Test login endpoint
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/login").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Test callback endpoint
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/callback?code=test_code&state=test_state")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key(SET_COOKIE));

        // Extract session token from cookie
        let cookie = response.headers()
            .get(SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        // Test logout endpoint
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/logout")
                    .header("Cookie", cookie)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        assert!(response.headers().contains_key(SET_COOKIE));

        // Clean up
        cleanup_test_data(pool).await;
    }
}
