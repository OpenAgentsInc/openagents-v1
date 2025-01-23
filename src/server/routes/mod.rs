use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::server::ws::handlers::chat::ChatHandler;
use crate::server::ws::types::WebSocketState;
use crate::server::services::deepseek::DeepSeekService;
use crate::server::tools::ToolExecutorFactory;
use crate::nostr::axum_relay::ws_handler;

pub mod chat;

pub fn routes() -> Router {
    let cors = CorsLayer::permissive();
    let ws_state = Arc::new(WebSocketState::new());
    let chat_handler = Arc::new(ChatHandler::new(
        ws_state.clone(),
        Arc::new(DeepSeekService::new("".to_string())),
        Arc::new(ToolExecutorFactory::new()),
    ));

    Router::new()
        .route("/ws", axum::routing::get(ws_handler))
        .merge(chat::chat_routes().with_state(chat_handler))
        .with_state(ws_state)
        .layer(cors)
}