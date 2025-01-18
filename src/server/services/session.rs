use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("Session not found")]
    NotFound,
    #[error("Session expired")]
    Expired,
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Failed to generate token")]
    TokenGeneration,
}

impl From<SessionError> for StatusCode {
    fn from(error: SessionError) -> Self {
        match error {
            SessionError::NotFound | SessionError::Expired => StatusCode::UNAUTHORIZED,
            SessionError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            SessionError::TokenGeneration => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Session {
    /// Generate a cryptographically secure random token
    fn generate_token() -> Result<String, SessionError> {
        let mut rng = rand::thread_rng();
        let token: String = std::iter::repeat_with(|| {
            const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .take(32)
        .collect();
        
        Ok(token)
    }

    /// Create a new session for a user
    pub async fn create(user_id: Uuid, pool: &PgPool) -> Result<Self, SessionError> {
        let token = Self::generate_token()?;
        let expires_at = Utc::now() + Duration::hours(24); // 24 hour expiry

        let session = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, expires_at, created_at, updated_at
            "#,
            user_id,
            token,
            expires_at,
        )
        .fetch_one(pool)
        .await?;

        Ok(session)
    }

    /// Validate a session token and return the session if valid
    pub async fn validate(token: &str, pool: &PgPool) -> Result<Self, SessionError> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, token, expires_at, created_at, updated_at
            FROM sessions
            WHERE token = $1
            "#,
            token,
        )
        .fetch_optional(pool)
        .await?
        .ok_or(SessionError::NotFound)?;

        if session.expires_at < Utc::now() {
            return Err(SessionError::Expired);
        }

        Ok(session)
    }

    /// Refresh a session by extending its expiry time
    pub async fn refresh(&mut self, pool: &PgPool) -> Result<(), SessionError> {
        self.expires_at = Utc::now() + Duration::hours(24);
        self.updated_at = Utc::now();

        sqlx::query!(
            r#"
            UPDATE sessions
            SET expires_at = $1, updated_at = $2
            WHERE id = $3
            "#,
            self.expires_at,
            self.updated_at,
            self.id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Delete a session
    pub async fn delete(self, pool: &PgPool) -> Result<(), SessionError> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE id = $1
            "#,
            self.id,
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    /// Clean up expired sessions
    pub async fn cleanup_expired(pool: &PgPool) -> Result<u64, SessionError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < $1
            "#,
            Utc::now(),
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    async fn setup_test_db() -> PgPool {
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");
        
        PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to connect to test database")
    }

    #[tokio::test]
    async fn test_session_lifecycle() {
        let pool = setup_test_db().await;
        
        // Create a test user
        let user_id = Uuid::new_v4();
        
        // Create session
        let session = Session::create(user_id, &pool)
            .await
            .expect("Failed to create session");
        
        assert_eq!(session.user_id, user_id);
        assert!(session.expires_at > Utc::now());
        
        // Validate session
        let validated = Session::validate(&session.token, &pool)
            .await
            .expect("Failed to validate session");
        
        assert_eq!(validated.id, session.id);
        assert_eq!(validated.user_id, user_id);
        
        // Refresh session
        let mut session = validated;
        let original_expiry = session.expires_at;
        session.refresh(&pool)
            .await
            .expect("Failed to refresh session");
        
        assert!(session.expires_at > original_expiry);
        
        // Delete session
        session.delete(&pool)
            .await
            .expect("Failed to delete session");
        
        // Verify session is gone
        let result = Session::validate(&session.token, &pool).await;
        assert!(matches!(result, Err(SessionError::NotFound)));
    }

    #[tokio::test]
    async fn test_expired_session() {
        let pool = setup_test_db().await;
        let user_id = Uuid::new_v4();
        
        // Create an expired session directly in the database
        let token = Session::generate_token().unwrap();
        let expired_at = Utc::now() - Duration::hours(1);
        
        sqlx::query!(
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            token,
            expired_at,
        )
        .execute(&pool)
        .await
        .expect("Failed to insert expired session");
        
        // Attempt to validate expired session
        let result = Session::validate(&token, &pool).await;
        assert!(matches!(result, Err(SessionError::Expired)));
        
        // Clean up expired sessions
        let cleaned = Session::cleanup_expired(&pool)
            .await
            .expect("Failed to clean up expired sessions");
        
        assert!(cleaned > 0);
    }
}