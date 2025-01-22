use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::server::ws::handlers::chat::ChatHandler;
use crate::server::ws::types::WebSocketState;
use crate::server::services::deepseek::DeepSeekService;
use crate::server::tools::ToolExecutorFactory;

pub mod chat;

pub fn routes() -> Router {
    let cors = CorsLayer::permissive();
    let chat_handler = Arc::new(ChatHandler::new(
        Arc::new(WebSocketState::new()),
        Arc::new(DeepSeekService::new("".to_string())),
        Arc::new(ToolExecutorFactory::new()),
    ));

    Router::new()
        .merge(chat::chat_routes().with_state(chat_handler))
        .layer(cors)
}