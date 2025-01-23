use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::server::ws::handlers::chat::ChatHandler;
use crate::server::ws::types::WebSocketState;
use crate::server::services::deepseek::DeepSeekService;
use crate::server::tools::ToolExecutorFactory;
use crate::nostr::axum_relay::{ws_handler, RelayState};
use tokio::sync::broadcast;

pub mod chat;

pub fn routes() -> Router {
    let cors = CorsLayer::permissive();
    
    // Initialize WebSocket state
    let ws_state = Arc::new(WebSocketState::new());
    
    // Initialize chat handler
    let chat_handler = Arc::new(ChatHandler::new(
        ws_state.clone(),
        Arc::new(DeepSeekService::new("".to_string())),
        Arc::new(ToolExecutorFactory::new()),
    ));

    // Initialize Nostr components
    let (event_tx, _) = broadcast::channel(1024);
    let relay_state = Arc::new(RelayState::new(event_tx));

    Router::new()
        .route("/ws", axum::routing::get(ws_handler))
        .merge(chat::chat_routes().with_state(chat_handler))
        .with_state(relay_state)
        .layer(cors)
}