use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::server::ws::handlers::chat::ChatHandlerService;

pub mod chat;

pub fn routes() -> Router {
    let cors = CorsLayer::permissive();
    let chat_handler = Arc::new(crate::server::ws::handlers::chat::ChatHandler::new(
        Arc::new(crate::server::ws::WebSocketState::new()),
        Arc::new(crate::server::services::DeepSeekService::new("".to_string())),
        Arc::new(crate::server::tools::ToolExecutorFactory::new()),
    ));

    Router::new()
        .merge(chat::chat_routes().with_state(chat_handler))
        .layer(cors)
}