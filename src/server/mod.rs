use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::nostr::db::Database;
use crate::configuration::get_configuration;

pub mod admin;
pub mod routes;
pub mod services;
pub mod tools;
pub mod ws;

pub async fn app() -> Router {
    let cors = CorsLayer::permissive();

    // Initialize database
    let configuration = get_configuration().expect("Failed to read configuration");
    let db = Arc::new(
        Database::new_with_options(configuration.database.connect_options())
            .await
            .expect("Failed to connect to database"),
    );

    Router::new()
        .merge(routes::routes_with_db(db))
        .layer(cors)
}