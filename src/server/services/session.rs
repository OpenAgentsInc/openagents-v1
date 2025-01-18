use axum::http::StatusCode;
use chrono::{DateTime, Duration, Utc};
use rand::Rng;
use serde::{Serialize};
use sqlx::PgPool;
use uuid::Uuid;
use time::OffsetDateTime;

#[derive(Debug, thiserror::Error, Clone)]
pub enum SessionError {
    #[error("Session not found")]
    NotFound,
    #[error("Session expired")]
    Expired,
    #[error("Database error: {0}")]
    Database(String),
    #[error("Failed to generate token")]
    TokenGeneration,
}

impl From<sqlx::Error> for SessionError {
    fn from(err: sqlx::Error) -> Self {
        SessionError::Database(err.to_string())
    }
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

#[derive(Debug, Serialize)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub expires_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
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

    fn to_offset_datetime(dt: DateTime<Utc>) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(dt.timestamp()).unwrap()
    }

    fn from_offset_datetime(odt: OffsetDateTime) -> DateTime<Utc> {
        DateTime::from_timestamp(odt.unix_timestamp(), 0).unwrap()
    }

    /// Create a new session for a user
    pub async fn create(user_id: Uuid, pool: &PgPool) -> Result<Self, SessionError> {
        let token = Self::generate_token()?;
        let expires_at = Utc::now() + Duration::hours(24); // 24 hour expiry

        let expires_at_offset = Self::to_offset_datetime(expires_at);

        let row = sqlx::query!(
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, expires_at, created_at, updated_at
            "#,
            user_id,
            token,
            expires_at_offset,
        )
        .fetch_one(pool)
        .await?;

        Ok(Session {
            id: row.id,
            user_id: row.user_id,
            token: row.token,
            expires_at: Self::from_offset_datetime(row.expires_at),
            created_at: Self::from_offset_datetime(row.created_at),
            updated_at: Self::from_offset_datetime(row.updated_at),
        })
    }

    /// Validate a session token and return the session if valid
    pub async fn validate(token: &str, pool: &PgPool) -> Result<Self, SessionError> {
        let row = sqlx::query!(
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

        let session = Session {
            id: row.id,
            user_id: row.user_id,
            token: row.token,
            expires_at: Self::from_offset_datetime(row.expires_at),
            created_at: Self::from_offset_datetime(row.created_at),
            updated_at: Self::from_offset_datetime(row.updated_at),
        };

        if session.expires_at < Utc::now() {
            return Err(SessionError::Expired);
        }

        Ok(session)
    }

    /// Refresh a session by extending its expiry time
    pub async fn refresh(&mut self, pool: &PgPool) -> Result<(), SessionError> {
        self.expires_at = Utc::now() + Duration::hours(24);
        self.updated_at = Utc::now();

        let expires_at_offset = Self::to_offset_datetime(self.expires_at);
        let updated_at_offset = Self::to_offset_datetime(self.updated_at);

        sqlx::query!(
            r#"
            UPDATE sessions
            SET expires_at = $1, updated_at = $2
            WHERE id = $3
            "#,
            expires_at_offset,
            updated_at_offset,
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
        let now = Self::to_offset_datetime(Utc::now());

        let result = sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE expires_at < $1
            "#,
            now,
        )
        .execute(pool)
        .await?;

        Ok(result.rows_affected())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::services::test_helpers::{get_test_pool, cleanup_test_data};
    use sqlx::query;

    async fn create_test_user(pool: &PgPool) -> Uuid {
        query!(
            r#"
            INSERT INTO users (pseudonym)
            VALUES ($1)
            RETURNING id
            "#,
            "test_user",
        )
        .fetch_one(pool)
        .await
        .unwrap()
        .id
    }

    #[tokio::test]
    async fn test_session_lifecycle() {
        let pool = get_test_pool().await;
        cleanup_test_data(pool).await;
        
        // Create a test user
        let user_id = create_test_user(pool).await;
        
        // Create session
        let session = Session::create(user_id, pool)
            .await
            .expect("Failed to create session");
        
        assert_eq!(session.user_id, user_id);
        assert!(session.expires_at > Utc::now());
        
        // Validate session
        let validated = Session::validate(&session.token, pool)
            .await
            .expect("Failed to validate session");
        
        assert_eq!(validated.id, session.id);
        assert_eq!(validated.user_id, user_id);
        
        // Refresh session
        let mut session = validated;
        let original_expiry = session.expires_at;
        session.refresh(pool)
            .await
            .expect("Failed to refresh session");
        
        assert!(session.expires_at > original_expiry);
        
        // Delete session
        session.delete(pool)
            .await
            .expect("Failed to delete session");
        
        // Verify session is gone
        let result = Session::validate(&session.token, pool).await;
        assert!(matches!(result, Err(SessionError::NotFound)));
    }

    #[tokio::test]
    async fn test_expired_session() {
        let pool = get_test_pool().await;
        cleanup_test_data(pool).await;
        
        // Create a test user
        let user_id = create_test_user(pool).await;
        
        // Create an expired session directly in the database
        let token = Session::generate_token().unwrap();
        let expired_at = Utc::now() - Duration::hours(1);
        let expired_at_offset = Session::to_offset_datetime(expired_at);
        
        sqlx::query!(
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            token,
            expired_at_offset,
        )
        .execute(pool)
        .await
        .expect("Failed to insert expired session");
        
        // Attempt to validate expired session
        let result = Session::validate(&token, pool).await;
        assert!(matches!(result, Err(SessionError::Expired)));
        
        // Clean up expired sessions
        let cleaned = Session::cleanup_expired(pool)
            .await
            .expect("Failed to clean up expired sessions");
        
        assert!(cleaned > 0);
    }

    #[tokio::test]
    async fn test_multiple_sessions() {
        let pool = get_test_pool().await;
        cleanup_test_data(pool).await;
        
        // Create test user
        let user_id = create_test_user(pool).await;
        
        // Create multiple sessions for the same user
        let session1 = Session::create(user_id, pool).await.unwrap();
        let session2 = Session::create(user_id, pool).await.unwrap();
        
        // Validate both sessions work
        let validated1 = Session::validate(&session1.token, pool).await.unwrap();
        let validated2 = Session::validate(&session2.token, pool).await.unwrap();
        
        assert_eq!(validated1.user_id, user_id);
        assert_eq!(validated2.user_id, user_id);
        assert_ne!(validated1.id, validated2.id);
        
        // Delete one session
        validated1.delete(pool).await.unwrap();
        
        // Verify only the deleted session is gone
        let result1 = Session::validate(&session1.token, pool).await;
        let result2 = Session::validate(&session2.token, pool).await;
        
        assert!(matches!(result1, Err(SessionError::NotFound)));
        assert!(result2.is_ok());
    }
}