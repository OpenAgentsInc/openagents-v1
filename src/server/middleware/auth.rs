use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use sqlx::PgPool;

use crate::server::services::{
    auth::{OIDCConfig, User},
    session::Session,
};

#[derive(Clone)]
pub struct AuthState {
    pub config: OIDCConfig,
    pub pool: PgPool,
}

impl AuthState {
    pub fn new(config: OIDCConfig, pool: PgPool) -> Self {
        Self { config, pool }
    }
}

#[derive(Clone)]
pub struct AuthenticatedUser {
    pub id: uuid::Uuid,
    pub pseudonym: String,
}

impl From<User> for AuthenticatedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            pseudonym: user.pseudonym,
        }
    }
}

pub async fn require_auth<B>(
    Extension(state): Extension<AuthState>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // Get session token from cookie
    let cookies = axum_extra::extract::cookie::CookieJar::from_headers(request.headers());
    let session_token = cookies
        .get("session")
        .ok_or(StatusCode::UNAUTHORIZED)?
        .value()
        .to_string();

    // Validate session and get user
    let user = state
        .config
        .validate_session(&session_token, &state.pool)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Add authenticated user to request extensions
    request.extensions_mut().insert(AuthenticatedUser::from(user));

    // Continue with the request
    Ok(next.run(request).await)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::services::test_helpers::{get_test_pool, begin_test_transaction};
    use axum::{
        body::Body,
        http::{header::COOKIE, Request},
        response::IntoResponse,
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    async fn test_handler(Extension(user): Extension<AuthenticatedUser>) -> impl IntoResponse {
        format!("Hello, {}!", user.pseudonym)
    }

    #[tokio::test]
    async fn test_auth_middleware() {
        // Setup test database
        let pool = get_test_pool().await;
        let mut tx = begin_test_transaction(pool).await;

        // Create test config
        let config = OIDCConfig::new(
            "client123".to_string(),
            "secret456".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
            "https://auth.scramble.com/.well-known/jwks.json".to_string(),
        )
        .unwrap();

        // Create test user and session
        let user_id = sqlx::query!(
            "INSERT INTO users (pseudonym) VALUES ($1) RETURNING id",
            "test_user"
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap()
        .id;

        let session_token = uuid::Uuid::new_v4().to_string();
        let expires_at = time::OffsetDateTime::now_utc() + time::Duration::hours(1);

        sqlx::query!(
            "INSERT INTO sessions (user_id, token, expires_at) VALUES ($1, $2, $3)",
            user_id,
            session_token,
            expires_at
        )
        .execute(&mut *tx)
        .await
        .unwrap();

        // Create test app
        let state = AuthState::new(config, pool.clone());
        let app = Router::new()
            .route("/protected", get(test_handler))
            .layer(axum::middleware::from_fn(require_auth))
            .layer(Extension(state));

        // Test without session cookie
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/protected").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test with invalid session token
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header(COOKIE, "session=invalid_token")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test with valid session token
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/protected")
                    .header(COOKIE, format!("session={}", session_token))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Rollback transaction
        tx.rollback().await.unwrap();
    }
}