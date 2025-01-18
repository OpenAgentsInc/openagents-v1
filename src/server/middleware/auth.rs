use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    Json,
};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use sqlx::PgPool;

use crate::server::services::{session::{Session, SessionError}, auth::User};
use crate::server::handlers::auth::AppState;

const SESSION_COOKIE_NAME: &str = "session";

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
    pub session: Session,
}

#[derive(Debug, thiserror::Error)]
pub enum AuthError {
    #[error("Not authenticated")]
    NotAuthenticated,
    #[error("Session error: {0}")]
    SessionError(#[from] SessionError),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            AuthError::NotAuthenticated => StatusCode::UNAUTHORIZED,
            AuthError::SessionError(ref e) => (*e).clone().into(),
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[async_trait::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts<'a>(parts: &'a mut Parts, state: &'a S) -> Result<Self, Self::Rejection> {
        // Get cookies from the request
        let cookies = CookieJar::from_headers(&parts.headers);

        // Get session token from cookie
        let session_token = cookies
            .get(SESSION_COOKIE_NAME)
            .ok_or(AuthError::NotAuthenticated)?
            .value()
            .to_string();

        // Get app state
        let app_state = parts.extensions.get::<AppState>()
            .ok_or(AuthError::NotAuthenticated)?;

        // Validate session
        let session = Session::validate(&session_token, &app_state.pool)
            .await
            .map_err(AuthError::SessionError)?;

        // Get user from database
        let user = sqlx::query_as!(
            User,
            "SELECT id, pseudonym FROM users WHERE id = $1",
            session.user_id
        )
        .fetch_optional(&app_state.pool)
        .await
        .map_err(|e| AuthError::SessionError(SessionError::Database(e.to_string())))?
        .ok_or(AuthError::NotAuthenticated)?;

        Ok(AuthenticatedUser { user, session })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::services::test_helpers::{get_test_pool, cleanup_test_data, setup_test_db};
    use axum::{
        body::Body,
        http::Request,
        response::IntoResponse,
        routing::get,
        Router,
        debug_handler,
    };
    use axum_extra::extract::cookie::Cookie;
    use tower::ServiceExt;

    #[debug_handler(state = AppState)]
    async fn test_handler(user: AuthenticatedUser) -> impl IntoResponse {
        Json(user.user)
    }

    #[tokio::test]
    async fn test_auth_middleware() {
        let pool = get_test_pool().await;
        setup_test_db(pool).await;
        cleanup_test_data(pool).await;

        // Create test user and session
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (pseudonym)
            VALUES ($1)
            RETURNING id, pseudonym
            "#,
            "test_user",
        )
        .fetch_one(pool)
        .await
        .unwrap();

        let session = Session::create(user.id, pool).await.unwrap();

        // Create app state
        let state = AppState::new(
            crate::server::services::auth::OIDCConfig::new(
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
                "test".to_string(),
            )
            .unwrap(),
            pool.clone(),
        );

        // Create test app
        let app = Router::new()
            .route("/test", get(test_handler))
            .with_state(state);

        // Test without session cookie
        let response = app
            .clone()
            .oneshot(Request::builder().uri("/test").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test with invalid session cookie
        let cookie = Cookie::new(SESSION_COOKIE_NAME, "invalid_token");
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/test")
                    .header("Cookie", cookie.to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

        // Test with valid session cookie
        let cookie = Cookie::new(SESSION_COOKIE_NAME, &session.token);
        let response = app
            .oneshot(
                Request::builder()
                    .uri("/test")
                    .header("Cookie", cookie.to_string())
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        // Clean up
        cleanup_test_data(pool).await;
    }
}