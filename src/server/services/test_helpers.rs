use sqlx::PgPool;
use std::env;
use tokio::sync::OnceCell;

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
    // Clean up test data in reverse order of foreign key dependencies
    let _ = sqlx::query!("DELETE FROM sessions").execute(pool).await;
    let _ = sqlx::query!("DELETE FROM users").execute(pool).await;
}

pub async fn setup_test_db(pool: &PgPool) {
    // Create tables if they don't exist
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

    // Create updated_at trigger function if it doesn't exist
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

    // Create triggers if they don't exist
    sqlx::query!(
        r#"
        DROP TRIGGER IF EXISTS users_updated_at ON users;
        CREATE TRIGGER users_updated_at
            BEFORE UPDATE ON users
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at();
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create users updated_at trigger");

    sqlx::query!(
        r#"
        DROP TRIGGER IF EXISTS sessions_updated_at ON sessions;
        CREATE TRIGGER sessions_updated_at
            BEFORE UPDATE ON sessions
            FOR EACH ROW
            EXECUTE FUNCTION update_updated_at();
        "#
    )
    .execute(pool)
    .await
    .expect("Failed to create sessions updated_at trigger");
}