use sqlx::PgPool;
use std::env;
use tokio::sync::OnceCell;
use lazy_static::lazy_static;
use tokio::sync::Mutex;

lazy_static! {
    static ref TEST_MUTEX: Mutex<()> = Mutex::new(());
}

static TEST_DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_test_pool() -> &'static PgPool {
    TEST_DB_POOL.get_or_init(|| async {
        let database_url = env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/openagents_test".to_string());
        
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(5) // Increased for better concurrent test handling
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool")
    }).await
}

/// Drops all test database objects in the correct order
pub async fn cleanup_test_data(pool: &PgPool) {
    let _lock = TEST_MUTEX.lock().await;

    // Drop triggers first
    let _ = sqlx::query!("DROP TRIGGER IF EXISTS sessions_updated_at ON sessions")
        .execute(pool)
        .await;
    let _ = sqlx::query!("DROP TRIGGER IF EXISTS users_updated_at ON users")
        .execute(pool)
        .await;

    // Drop tables (in correct order due to foreign keys)
    let _ = sqlx::query!("DROP TABLE IF EXISTS sessions CASCADE")
        .execute(pool)
        .await;
    let _ = sqlx::query!("DROP TABLE IF EXISTS users CASCADE")
        .execute(pool)
        .await;

    // Drop functions last
    let _ = sqlx::query!("DROP FUNCTION IF EXISTS update_updated_at() CASCADE")
        .execute(pool)
        .await;
}

/// Creates the test database schema in the correct order
pub async fn setup_test_db(pool: &PgPool) {
    let _lock = TEST_MUTEX.lock().await;

    // First ensure clean state
    cleanup_test_data(pool).await;

    // Create the updated_at trigger function first
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

    // Create users updated_at trigger
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

    // Create sessions updated_at trigger
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

/// Helper function to create a test user with a specific pseudonym
pub async fn create_test_user(pool: &PgPool, pseudonym: &str) -> uuid::Uuid {
    let _lock = TEST_MUTEX.lock().await;
    
    sqlx::query!(
        r#"
        INSERT INTO users (pseudonym)
        VALUES ($1)
        RETURNING id
        "#,
        pseudonym,
    )
    .fetch_one(pool)
    .await
    .expect("Failed to create test user")
    .id
}

/// Helper function to create a test session for a user
pub async fn create_test_session(pool: &PgPool, user_id: uuid::Uuid) -> String {
    let _lock = TEST_MUTEX.lock().await;
    
    let token = uuid::Uuid::new_v4().to_string();
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(1);

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
    .fetch_one(pool)
    .await
    .expect("Failed to create test session")
    .token
}

/// Helper function to cleanup all test data for a specific user
pub async fn cleanup_test_user(pool: &PgPool, user_id: uuid::Uuid) {
    let _lock = TEST_MUTEX.lock().await;
    
    // Sessions will be deleted automatically due to ON DELETE CASCADE
    sqlx::query!(
        "DELETE FROM users WHERE id = $1",
        user_id
    )
    .execute(pool)
    .await
    .expect("Failed to cleanup test user");
}