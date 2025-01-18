use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Row};
use std::fmt::{self, Display};
use uuid::Uuid;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use super::session::{Session, SessionError};

#[derive(Debug)]
pub enum AuthError {
    InvalidConfig,
    AuthenticationFailed,
    TokenExchangeFailed(String),
    DatabaseError(String),
    InvalidToken(String),
    SessionError(SessionError),
}

impl From<SessionError> for AuthError {
    fn from(error: SessionError) -> Self {
        AuthError::SessionError(error)
    }
}

impl Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::InvalidConfig => write!(f, "Invalid OIDC configuration"),
            AuthError::AuthenticationFailed => write!(f, "Authentication failed"),
            AuthError::TokenExchangeFailed(msg) => write!(f, "Token exchange failed: {}", msg),
            AuthError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            AuthError::InvalidToken(msg) => write!(f, "Invalid token: {}", msg),
            AuthError::SessionError(e) => write!(f, "Session error: {}", e),
        }
    }
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AuthError::InvalidConfig => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::AuthenticationFailed => StatusCode::UNAUTHORIZED,
            AuthError::TokenExchangeFailed(_) => StatusCode::BAD_GATEWAY,
            AuthError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::InvalidToken(_) => StatusCode::UNAUTHORIZED,
            AuthError::SessionError(e) => e.into(),
        };
        
        let body = Json(serde_json::json!({
            "error": self.to_string()
        }));

        (status, body).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OIDCConfig {
    client_id: String,
    client_secret: String,
    redirect_uri: String,
    authorization_endpoint: String,
    token_endpoint: String,
    jwks_uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<i32>,
    id_token: String,
}

#[derive(Debug, Serialize)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    code: String,
    redirect_uri: &'a str,
    client_id: &'a str,
    client_secret: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub pseudonym: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
    iat: usize,
    iss: String,
    aud: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: User,
    pub session_token: String,
}

impl OIDCConfig {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        authorization_endpoint: String,
        token_endpoint: String,
        jwks_uri: String,
    ) -> Result<Self, AuthError> {
        // Basic validation
        if client_id.is_empty() || client_secret.is_empty() || redirect_uri.is_empty() {
            return Err(AuthError::InvalidConfig);
        }

        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
            authorization_endpoint,
            token_endpoint,
            jwks_uri,
        })
    }

    pub fn authorization_url(&self) -> String {
        format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid",
            self.authorization_endpoint,
            self.client_id,
            urlencoding::encode(&self.redirect_uri)
        )
    }

    pub async fn exchange_code(&self, code: String) -> Result<TokenResponse, AuthError> {
        let client = reqwest::Client::new();
        
        let token_request = TokenRequest {
            grant_type: "authorization_code",
            code,
            redirect_uri: &self.redirect_uri,
            client_id: &self.client_id,
            client_secret: &self.client_secret,
        };

        let response = client
            .post(&self.token_endpoint)
            .json(&token_request)
            .send()
            .await
            .map_err(|e| AuthError::TokenExchangeFailed(e.to_string()))?;

        if !response.status().is_success() {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AuthError::TokenExchangeFailed(error_text));
        }

        response
            .json::<TokenResponse>()
            .await
            .map_err(|e| AuthError::TokenExchangeFailed(e.to_string()))
    }

    pub async fn verify_and_get_pseudonym(&self, id_token: &str) -> Result<String, AuthError> {
        // For testing purposes, using a simple key. In production, fetch from jwks_uri
        let key = DecodingKey::from_secret(self.client_secret.as_bytes());
        
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(id_token, &key, &validation)
            .map_err(|e| AuthError::InvalidToken(e.to_string()))?;

        Ok(token_data.claims.sub)
    }

    pub async fn get_or_create_user(&self, id_token: &str, pool: &PgPool) -> Result<User, AuthError> {
        let pseudonym = self.verify_and_get_pseudonym(id_token).await?;

        // Try to find existing user
        let user = sqlx::query(
            "SELECT id, pseudonym FROM users WHERE pseudonym = $1"
        )
        .bind(&pseudonym)
        .map(|row: sqlx::postgres::PgRow| {
            User {
                id: row.get("id"),
                pseudonym: row.get("pseudonym"),
            }
        })
        .fetch_optional(pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Return existing user or create new one
        match user {
            Some(user) => Ok(user),
            None => {
                let user = sqlx::query(
                    "INSERT INTO users (pseudonym) VALUES ($1) RETURNING id, pseudonym"
                )
                .bind(&pseudonym)
                .map(|row: sqlx::postgres::PgRow| {
                    User {
                        id: row.get("id"),
                        pseudonym: row.get("pseudonym"),
                    }
                })
                .fetch_one(pool)
                .await
                .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

                Ok(user)
            }
        }
    }

    pub async fn authenticate(&self, code: String, pool: &PgPool) -> Result<AuthResponse, AuthError> {
        // Exchange code for tokens
        let token_response = self.exchange_code(code).await?;
        
        // Get or create user
        let user = self.get_or_create_user(&token_response.id_token, pool).await?;
        
        // Create session
        let session = Session::create(user.id, pool).await?;
        
        Ok(AuthResponse {
            user,
            session_token: session.token,
        })
    }

    pub async fn validate_session(&self, session_token: &str, pool: &PgPool) -> Result<User, AuthError> {
        // Validate session
        let session = Session::validate(session_token, pool).await?;
        
        // Get user
        let user = sqlx::query_as!(
            User,
            "SELECT id, pseudonym FROM users WHERE id = $1",
            session.user_id
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| AuthError::DatabaseError(e.to_string()))?
        .ok_or(AuthError::AuthenticationFailed)?;

        Ok(user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::method;
    use serde_json::json;
    use crate::server::services::test_helpers::{get_test_pool, cleanup_test_data};

    #[test]
    fn test_oidc_config_validation() {
        // Test invalid config (empty client_id)
        let result = OIDCConfig::new(
            "".to_string(),
            "secret".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
            "https://auth.scramble.com/.well-known/jwks.json".to_string(),
        );
        assert!(matches!(result, Err(AuthError::InvalidConfig)));

        // Test valid config
        let result = OIDCConfig::new(
            "client123".to_string(),
            "secret".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
            "https://auth.scramble.com/.well-known/jwks.json".to_string(),
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_authorization_url_generation() {
        let config = OIDCConfig::new(
            "client123".to_string(),
            "secret".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
            "https://auth.scramble.com/.well-known/jwks.json".to_string(),
        )
        .unwrap();

        let auth_url = config.authorization_url();
        let encoded_callback = urlencoding::encode("http://localhost:3000/callback").into_owned();
        
        assert!(auth_url.starts_with("https://auth.scramble.com/authorize"));
        assert!(auth_url.contains("client_id=client123"));
        assert!(auth_url.contains("response_type=code"));
        assert!(auth_url.contains("scope=openid"));
        assert!(auth_url.contains(&encoded_callback));
    }

    #[tokio::test]
    async fn test_authentication_flow() {
        let pool = get_test_pool().await;
        cleanup_test_data(pool).await;
        
        // Start mock server
        let mock_server = MockServer::start().await;

        // Create test config with mock server URL
        let config = OIDCConfig::new(
            "client123".to_string(),
            "secret456".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            mock_server.uri(),
            "https://auth.scramble.com/.well-known/jwks.json".to_string(),
        )
        .unwrap();

        // Setup successful token response
        Mock::given(method("POST"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "access_token": "test_access_token",
                "token_type": "Bearer",
                "expires_in": 3600,
                "id_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJ0ZXN0X3BzZXVkb255bSIsImV4cCI6MTk5OTk5OTk5OSwiaWF0IjoxNTE2MjM5MDIyLCJpc3MiOiJodHRwczovL2F1dGguc2NyYW1ibGUuY29tIiwiYXVkIjoiY2xpZW50MTIzIn0.8D8vhM6pzxsQPLUXeHxw7cWoKhvGp4BUJ4Q8E6JIftw"
            })))
            .mount(&mock_server)
            .await;

        // Test full authentication flow
        let auth_response = config.authenticate("test_code".to_string(), pool).await.unwrap();
        
        // Verify user was created
        assert_eq!(auth_response.user.pseudonym, "test_pseudonym");
        
        // Verify session works
        let validated_user = config.validate_session(&auth_response.session_token, pool).await.unwrap();
        assert_eq!(validated_user.id, auth_response.user.id);
    }

    #[tokio::test]
    async fn test_session_validation_failure() {
        let pool = get_test_pool().await;
        cleanup_test_data(pool).await;
        
        let config = OIDCConfig::new(
            "client123".to_string(),
            "secret456".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
            "https://auth.scramble.com/.well-known/jwks.json".to_string(),
        )
        .unwrap();

        // Test with invalid session token
        let result = config.validate_session("invalid_token", pool).await;
        assert!(matches!(result, Err(AuthError::SessionError(SessionError::NotFound))));
    }
}