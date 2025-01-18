use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid OIDC configuration")]
    InvalidConfig,
    #[error("Authentication failed")]
    AuthenticationFailed,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AuthError::InvalidConfig => StatusCode::INTERNAL_SERVER_ERROR,
            AuthError::AuthenticationFailed => StatusCode::UNAUTHORIZED,
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
}

impl OIDCConfig {
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_uri: String,
        authorization_endpoint: String,
        token_endpoint: String,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oidc_config_validation() {
        // Test invalid config (empty client_id)
        let result = OIDCConfig::new(
            "".to_string(),
            "secret".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
        );
        assert!(matches!(result, Err(AuthError::InvalidConfig)));

        // Test valid config
        let result = OIDCConfig::new(
            "client123".to_string(),
            "secret".to_string(),
            "http://localhost:3000/callback".to_string(),
            "https://auth.scramble.com/authorize".to_string(),
            "https://auth.scramble.com/token".to_string(),
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
        )
        .unwrap();

        let auth_url = config.authorization_url();
        
        assert!(auth_url.starts_with("https://auth.scramble.com/authorize"));
        assert!(auth_url.contains("client_id=client123"));
        assert!(auth_url.contains("response_type=code"));
        assert!(auth_url.contains("scope=openid"));
        assert!(auth_url.contains(&urlencoding::encode("http://localhost:3000/callback")));
    }
}