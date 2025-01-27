use axum::{routing::post, Router};
use axum_test::TestServer;
use dotenvy::dotenv;
use openagents::server::{handlers::user::create_user, models::user::User};
use serde_json::json;
use sqlx::PgPool;
use time::Duration;
use tracing::{info, Level};
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::test]
async fn test_user_creation() {
    // Initialize logging with custom format
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_test_writer()
        .with_span_events(FmtSpan::NONE)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    // Load environment variables
    dotenv().ok();

    // Set up database connection
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Clean up any existing test data before starting
    sqlx::query!("DELETE FROM users WHERE scramble_id LIKE 'test_user_%'")
        .execute(&pool)
        .await
        .expect("Failed to clean up existing test data");

    // Create router with user creation endpoint
    let app = Router::new()
        .route("/users", post(create_user))
        .with_state(pool.clone());

    // Create test server
    let server = TestServer::new(app).unwrap();

    // Test cases with descriptions for better error messages
    let test_cases = vec![
        (
            "Valid user creation",
            json!({
                "scramble_id": "test_user_1",
                "metadata": {
                    "display_name": "Test User 1"
                }
            }),
            200,
        ),
        (
            "Duplicate scramble_id",
            json!({
                "scramble_id": "test_user_1",
                "metadata": {
                    "display_name": "Duplicate User"
                }
            }),
            409,
        ),
        (
            "Missing required field",
            json!({
                "metadata": {
                    "display_name": "Invalid User"
                }
            }),
            422,
        ),
    ];

    for (test_description, input, expected_status) in test_cases {
        info!("Testing {} ...", test_description);

        // Make request
        let response = server.post("/users").json(&input).await;

        // Assert status code
        assert_eq!(
            response.status_code(),
            expected_status,
            "{} - Status code mismatch for input: {}",
            test_description,
            input
        );

        // For successful creation, verify response
        if expected_status == 200 {
            let user: User = response.json();

            // Verify user fields
            assert_eq!(
                user.scramble_id,
                input["scramble_id"].as_str().unwrap(),
                "{} - scramble_id mismatch",
                test_description
            );

            // Verify metadata
            if let Some(metadata) = input.get("metadata") {
                assert_eq!(
                    user.metadata.as_ref().unwrap(),
                    metadata,
                    "{} - metadata mismatch",
                    test_description
                );
            }

            // Verify timestamps exist
            assert!(
                user.created_at.is_some(),
                "{} - created_at should be set",
                test_description
            );
            assert!(
                user.updated_at.is_some(),
                "{} - updated_at should be set",
                test_description
            );

            // Verify user exists in database
            let db_user = sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE scramble_id = $1",
                user.scramble_id
            )
            .fetch_one(&pool)
            .await
            .expect("User should exist in database");

            // Compare non-timestamp fields
            assert_eq!(user.id, db_user.id, "{} - id mismatch", test_description);
            assert_eq!(
                user.scramble_id, db_user.scramble_id,
                "{} - scramble_id mismatch",
                test_description
            );
            assert_eq!(
                user.metadata, db_user.metadata,
                "{} - metadata mismatch",
                test_description
            );

            // Verify timestamps are within 1 second of each other
            if let (Some(user_created), Some(db_created)) = (user.created_at, db_user.created_at) {
                assert!(
                    (user_created - db_created).abs() < Duration::seconds(1),
                    "{} - created_at timestamps differ by more than 1 second",
                    test_description
                );
            }

            if let (Some(user_updated), Some(db_updated)) = (user.updated_at, db_user.updated_at) {
                assert!(
                    (user_updated - db_updated).abs() < Duration::seconds(1),
                    "{} - updated_at timestamps differ by more than 1 second",
                    test_description
                );
            }

            info!("✓ {} succeeded", test_description);
        }

        // For error cases, verify error response
        if expected_status != 200 {
            let error: serde_json::Value = response.json();
            assert!(
                error.get("error").is_some(),
                "{} - Error response should contain error field",
                test_description
            );
            info!(
                "✓ {} failed as expected with: {}",
                test_description, error["error"]
            );
        }
    }

    // Clean up all test data at the end
    sqlx::query!("DELETE FROM users WHERE scramble_id LIKE 'test_user_%'")
        .execute(&pool)
        .await
        .expect("Failed to clean up test data");
}
