use sqlx::PgPool;
use std::env;
use tokio::sync::OnceCell;

static TEST_DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_test_pool() -> &'static PgPool {
    TEST_DB_POOL.get_or_init(|| async {
        let database_url = env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must be set");
        
        sqlx::postgres::PgPoolOptions::new()
            .max_connections(3)
            .connect(&database_url)
            .await
            .expect("Failed to create test database pool")
    }).await
}

pub async fn cleanup_test_data(pool: &PgPool) {
    // Clean up test data in reverse order of foreign key dependencies
    sqlx::query!("DELETE FROM sessions").execute(pool).await.unwrap();
    sqlx::query!("DELETE FROM users").execute(pool).await.unwrap();
}