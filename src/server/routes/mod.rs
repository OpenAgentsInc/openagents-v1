use axum::{Router, extract::WebSocketUpgrade};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::server::ws::{
    handlers::chat::ChatHandler,
    transport::WebSocketState,
    handler::ws_handler,
};
use crate::server::services::DeepSeekService;
use crate::nostr::axum_relay::RelayState;
use tokio::sync::broadcast;
use crate::nostr::db::Database;

pub mod chat;

async fn ws_route(
    ws: WebSocketUpgrade,
    ws_state: Arc<WebSocketState>,
    chat_handler: Arc<ChatHandler>,
) -> axum::response::Response {
    ws_handler(ws, ws_state, chat_handler).await
}

pub fn routes_with_db(db: Arc<Database>) -> Router {
    let cors = CorsLayer::permissive();
    
    // Initialize WebSocket state and DeepSeek service
    let deepseek_service = Arc::new(DeepSeekService::new("".to_string()));
    let ws_state = WebSocketState::new(deepseek_service.clone());
    
    // Create handlers
    let (chat_handler, solver_handler) = WebSocketState::create_handlers(ws_state.clone());

    // Initialize Nostr components
    let (event_tx, _) = broadcast::channel(1024);
    let relay_state = Arc::new(RelayState::new(event_tx, db));

    Router::new()
        .route("/ws", axum::routing::get(ws_route))
        .merge(chat::chat_routes().with_state(chat_handler.clone()))
        .with_state(relay_state)
        .with_state(ws_state.clone())
        .with_state(chat_handler)
        .layer(cors)
}