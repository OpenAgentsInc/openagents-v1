use axum::http::StatusCode;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub expires_at: OffsetDateTime,
}

#[derive(Debug, thiserror::Error, Clone)]
pub enum SessionError {
    #[error("Session not found")]
    NotFound,
    #[error("Session expired")]
    Expired,
    #[error("Database error: {0}")]
    Database(String),
}

impl From<SessionError> for StatusCode {
    fn from(error: SessionError) -> Self {
        match error {
            SessionError::NotFound => StatusCode::UNAUTHORIZED,
            SessionError::Expired => StatusCode::UNAUTHORIZED,
            SessionError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl Session {
    pub async fn create(user_id: Uuid, pool: &PgPool) -> Result<Self, SessionError> {
        let token = Uuid::new_v4().to_string();
        let expires_at = OffsetDateTime::now_utc() + time::Duration::hours(24);

        let session = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            RETURNING id, user_id, token, expires_at
            "#,
            user_id,
            token,
            expires_at,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| SessionError::Database(e.to_string()))?;

        Ok(session)
    }

    pub async fn validate(token: &str, pool: &PgPool) -> Result<Self, SessionError> {
        let session = sqlx::query_as!(
            Session,
            r#"
            SELECT id, user_id, token, expires_at
            FROM sessions
            WHERE token = $1
            "#,
            token,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| SessionError::Database(e.to_string()))?
        .ok_or(SessionError::NotFound)?;

        if session.expires_at < OffsetDateTime::now_utc() {
            return Err(SessionError::Expired);
        }

        Ok(session)
    }

    pub async fn delete(self, pool: &PgPool) -> Result<(), SessionError> {
        sqlx::query!(
            r#"
            DELETE FROM sessions
            WHERE id = $1
            "#,
            self.id,
        )
        .execute(pool)
        .await
        .map_err(|e| SessionError::Database(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::services::test_helpers::{get_test_pool, begin_test_transaction};

    #[tokio::test]
    async fn test_session_lifecycle() {
        let pool = get_test_pool().await;
        let mut tx = begin_test_transaction(pool).await;

        // Create test user
        let user_id = sqlx::query!(
            "INSERT INTO users (pseudonym) VALUES ($1) RETURNING id",
            "test_user"
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap()
        .id;

        // Create session
        let session = Session::create(user_id, &mut *tx).await.unwrap();

        // Validate session
        let validated = Session::validate(&session.token, &mut *tx).await.unwrap();
        assert_eq!(validated.id, session.id);
        assert_eq!(validated.user_id, user_id);

        // Delete session
        session.delete(&mut *tx).await.unwrap();

        // Verify session is deleted
        let result = Session::validate(&session.token, &mut *tx).await;
        assert!(matches!(result, Err(SessionError::NotFound)));

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_expired_session() {
        let pool = get_test_pool().await;
        let mut tx = begin_test_transaction(pool).await;

        // Create test user
        let user_id = sqlx::query!(
            "INSERT INTO users (pseudonym) VALUES ($1) RETURNING id",
            "test_user"
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap()
        .id;

        // Create expired session
        let token = Uuid::new_v4().to_string();
        let expires_at = OffsetDateTime::now_utc() - time::Duration::hours(1);

        sqlx::query!(
            r#"
            INSERT INTO sessions (user_id, token, expires_at)
            VALUES ($1, $2, $3)
            "#,
            user_id,
            token,
            expires_at,
        )
        .execute(&mut *tx)
        .await
        .unwrap();

        // Verify session is expired
        let result = Session::validate(&token, &mut *tx).await;
        assert!(matches!(result, Err(SessionError::Expired)));

        tx.rollback().await.unwrap();
    }

    #[tokio::test]
    async fn test_multiple_sessions() {
        let pool = get_test_pool().await;
        let mut tx = begin_test_transaction(pool).await;

        // Create test user
        let user_id = sqlx::query!(
            "INSERT INTO users (pseudonym) VALUES ($1) RETURNING id",
            "test_user"
        )
        .fetch_one(&mut *tx)
        .await
        .unwrap()
        .id;

        // Create multiple sessions
        let session1 = Session::create(user_id, &mut *tx).await.unwrap();
        let session2 = Session::create(user_id, &mut *tx).await.unwrap();

        // Validate both sessions
        let validated1 = Session::validate(&session1.token, &mut *tx).await.unwrap();
        let validated2 = Session::validate(&session2.token, &mut *tx).await.unwrap();

        assert_eq!(validated1.id, session1.id);
        assert_eq!(validated2.id, session2.id);
        assert_eq!(validated1.user_id, user_id);
        assert_eq!(validated2.user_id, user_id);

        // Delete one session
        session1.delete(&mut *tx).await.unwrap();

        // Verify first session is deleted but second still works
        let result1 = Session::validate(&session1.token, &mut *tx).await;
        let result2 = Session::validate(&session2.token, &mut *tx).await;

        assert!(matches!(result1, Err(SessionError::NotFound)));
        assert!(result2.is_ok());

        tx.rollback().await.unwrap();
    }
}