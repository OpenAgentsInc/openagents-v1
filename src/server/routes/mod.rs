use axum::Router;
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use crate::server::ws::{
    handlers::chat::ChatHandler,
    types::WebSocketState,
    transport::WebSocketTransport,
    handler::ws_handler,
};
use crate::server::services::deepseek::DeepSeekService;
use crate::server::tools::ToolExecutorFactory;
use crate::nostr::axum_relay::RelayState;
use tokio::sync::broadcast;
use crate::nostr::db::Database;

pub mod chat;

pub fn routes_with_db(db: Arc<Database>) -> Router {
    let cors = CorsLayer::permissive();
    
    // Initialize WebSocket state
    let ws_state = Arc::new(WebSocketState::new());
    
    // Initialize chat handler
    let chat_handler = Arc::new(ChatHandler::new(
        ws_state.clone(),
        Arc::new(DeepSeekService::new("".to_string())),
        Arc::new(ToolExecutorFactory::new()),
    ));

    // Initialize WebSocket transport
    let transport = Arc::new(WebSocketTransport::new(
        ws_state.clone(),
        chat_handler.clone(),
    ));

    // Initialize Nostr components
    let (event_tx, _) = broadcast::channel(1024);
    let relay_state = Arc::new(RelayState::new(event_tx, db));

    Router::new()
        .route(
            "/ws",
            axum::routing::get(|ws, ws_state: Arc<WebSocketState>, transport: Arc<WebSocketTransport>| {
                ws_handler(ws, ws_state, transport)
            }),
        )
        .merge(chat::chat_routes().with_state(chat_handler))
        .with_state(relay_state)
        .with_state(ws_state)
        .with_state(transport)
        .layer(cors)
}