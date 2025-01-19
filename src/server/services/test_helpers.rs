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
            .max_connections(3)
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool")
    }).await
}

pub async fn cleanup_test_data(pool: &PgPool) {
    // Drop everything in a transaction
    let mut tx = pool.begin().await.expect("Failed to start transaction");

    // Drop triggers first
    let _ = sqlx::query!("DROP TRIGGER IF EXISTS sessions_updated_at ON sessions")
        .execute(&mut tx)
        .await;
    let _ = sqlx::query!("DROP TRIGGER IF EXISTS users_updated_at ON users")
        .execute(&mut tx)
        .await;

    // Drop function
    let _ = sqlx::query!("DROP FUNCTION IF EXISTS update_updated_at() CASCADE")
        .execute(&mut tx)
        .await;

    // Clean up data
    let _ = sqlx::query!("DELETE FROM sessions").execute(&mut tx).await;
    let _ = sqlx::query!("DELETE FROM users").execute(&mut tx).await;

    // Drop and recreate tables to ensure clean state
    let _ = sqlx::query!("DROP TABLE IF EXISTS sessions CASCADE").execute(&mut tx).await;
    let _ = sqlx::query!("DROP TABLE IF EXISTS users CASCADE").execute(&mut tx).await;

    tx.commit().await.expect("Failed to commit cleanup transaction");
}

pub async fn setup_test_db(pool: &PgPool) {
    // Ensure we have a lock for setup
    let _lock = TEST_MUTEX.lock().await;
    
    // Use a transaction for the entire setup
    let mut tx = pool.begin().await.expect("Failed to start transaction");

    // Create tables
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
    .execute(&mut tx)
    .await
    .expect("Failed to create users table");

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
    .execute(&mut tx)
    .await
    .expect("Failed to create sessions table");

    // Create indexes
    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_sessions_token ON sessions(token)")
        .execute(&mut tx)
        .await
        .expect("Failed to create sessions token index");

    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_sessions_user_id ON sessions(user_id)")
        .execute(&mut tx)
        .await
        .expect("Failed to create sessions user_id index");

    sqlx::query!("CREATE INDEX IF NOT EXISTS idx_sessions_expires_at ON sessions(expires_at)")
        .execute(&mut tx)
        .await
        .expect("Failed to create sessions expires_at index");

    // Create updated_at trigger function
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
    .execute(&mut tx)
    .await
    .expect("Failed to create update_updated_at function");

    // Create triggers
    sqlx::query!(
        r#"
        CREATE TRIGGER users_updated_at
            BEFORE UPDATE ON users
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at()
        "#
    )
    .execute(&mut tx)
    .await
    .expect("Failed to create users updated_at trigger");

    sqlx::query!(
        r#"
        CREATE TRIGGER sessions_updated_at
            BEFORE UPDATE ON sessions
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at()
        "#
    )
    .execute(&mut tx)
    .await
    .expect("Failed to create sessions updated_at trigger");

    // Commit all changes
    tx.commit().await.expect("Failed to commit setup transaction");
}

// Helper function to create a test user with a specific pseudonym
pub async fn create_test_user(pool: &PgPool, pseudonym: &str) -> uuid::Uuid {
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