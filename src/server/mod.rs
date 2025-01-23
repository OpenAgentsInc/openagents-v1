use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::nostr::db::Database;
use crate::configuration::get_configuration;
use crate::server::services::deepseek::DeepSeekService;

pub mod admin;
pub mod routes;
pub mod services;
pub mod tools;
pub mod ws;

pub async fn app() -> Router {
    let cors = CorsLayer::permissive();

    // Load configuration
    let configuration = get_configuration().expect("Failed to read configuration");

    // Initialize database
    let db = Arc::new(
        Database::new_with_options(configuration.database.connect_options())
            .await
            .expect("Failed to connect to database"),
    );

    // Initialize DeepSeek service
    let deepseek_service = Arc::new(DeepSeekService::new(configuration.deepseek.api_key));

    Router::new()
        .merge(routes::routes_with_db(db))
        .layer(cors)
}