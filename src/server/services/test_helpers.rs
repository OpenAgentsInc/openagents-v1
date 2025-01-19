use sqlx::PgPool;
use std::env;
use tokio::sync::OnceCell;
use time::OffsetDateTime;
use std::sync::Once;

static INIT: Once = Once::new();
static TEST_DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

/// Initialize the test database schema
async fn initialize_schema(pool: &PgPool) {
    // Create the updated_at trigger function
    sqlx::query!(
        r#"
        CREATE OR REPLACE FUNCTION update_updated_at()
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = NOW();
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create update_updated_at function");

    // Create users table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            pseudonym VARCHAR(255) NOT NULL UNIQUE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create users table");

    // Drop users trigger first
    sqlx::query!("DROP TRIGGER IF EXISTS users_updated_at ON users")
        .execute(executor)
        .await
        .expect("Failed to drop users updated_at trigger");

    // Create users trigger
    sqlx::query!(
        r#"
        CREATE TRIGGER users_updated_at
            BEFORE UPDATE ON users
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at()
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create users updated_at trigger");

    // Create sessions table
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
            token VARCHAR(255) NOT NULL UNIQUE,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create sessions table");

    // Drop sessions trigger first
    sqlx::query!("DROP TRIGGER IF EXISTS sessions_updated_at ON sessions")
        .execute(pool)
        .await
        .expect("Failed to drop sessions updated_at trigger");

    // Create sessions trigger
    sqlx::query!(
        r#"
        CREATE TRIGGER sessions_updated_at
            BEFORE UPDATE ON sessions
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at()
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create sessions updated_at trigger");

    // Create indexes
    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token)")
        .execute(pool)
        .await
        .expect("Failed to create sessions token index");

    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id)")
        .execute(pool)
        .await
        .expect("Failed to create sessions user_id index");

    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at)")
        .execute(pool)
        .await
        .expect("Failed to create sessions expires_at index");
}

pub async fn get_test_pool() -> &'static PgPool {
    TEST_DB_POOL.get_or_init(|| async {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/openagents_test".to_string());
        
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool");

        // Initialize schema only once
        INIT.call_once(|| {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(async {
                    initialize_schema(&pool).await;
                });
        });

        pool
    })
    .await
}

/// Start a new test transaction
pub async fn begin_test_transaction(pool: &PgPool) -> sqlx::Transaction<'_, sqlx::Postgres> {
    pool.begin().await.expect("Failed to start test transaction")
}

/// Helper function to create a test user with a specific pseudonym
pub async fn create_test_user<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    pseudonym: &str,
) -> uuid::Uuid {
    sqlx::query!(
        r#"
        INSERT INTO users (pseudonym)
        VALUES ($1)
        RETURNING id
        "#,
        pseudonym,
    )
    .fetch_one(&mut **tx)
    .await
    .expect("Failed to create test user")
    .id
}

/// Helper function to create a test session for a user
pub async fn create_test_session<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    user_id: uuid::Uuid,
) -> String {
    let token = uuid::Uuid::new_v4().to_string();
    let expires_at = OffsetDateTime::now_utc() + time::Duration::hours(1);

    sqlx::query!(
        r#"
        INSERT INTO sessions (user_id, token, expires_at)
        VALUES ($1, $2, $3)
        RETURNING token
        "#,
        user_id,
        token,
        expires_at,
    )
    .fetch_one(&mut **tx)
    .await
    .expect("Failed to create test session")
    .token
}

/// Helper function to cleanup test data for a specific user
pub async fn cleanup_test_user<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    user_id: uuid::Uuid,
) {
    // Sessions will be deleted automatically due to ON DELETE CASCADE
    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(&mut **tx)
    .await
    .expect("Failed to cleanup test user");
}

// For backward compatibility with existing tests
pub async fn setup_test_db(_pool: &PgPool) {
    // No-op as schema is initialized in get_test_pool
}

pub async fn cleanup_test_data(pool: &PgPool) {
    // Clean up data but keep schema
    let mut tx = begin_test_transaction(pool).await;
    sqlx::query!("DELETE FROM sessions").execute(&mut *tx).await.unwrap();
    sqlx::query!("DELETE FROM users").execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_transaction_isolation() {
        let pool = get_test_pool().await;
        let mut tx1 = begin_test_transaction(pool).await;
        let mut tx2 = begin_test_transaction(pool).await;

        // Create user in tx1
        let user_id = create_test_user(&mut tx1, "test_user1").await;

        // Try to create same user in tx2 (should not see tx1's changes)
        let result = create_test_user(&mut tx2, "test_user1").await;
        assert_ne!(user_id, result);

        // Rollback both transactions
        tx1.rollback().await.unwrap();
        tx2.rollback().await.unwrap();
    }
}
