use axum::{Router, extract::{WebSocketUpgrade, State}};
use tower_http::cors::CorsLayer;
use std::{sync::Arc, env};
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
use tracing::warn;

pub mod chat;

// Define the app state struct
#[derive(Clone)]
struct AppState {
    ws_state: Arc<WebSocketState>,
    transport: Arc<WebSocketTransport>,
}

async fn ws_route(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> axum::response::Response {
    ws_handler(ws, state.ws_state, state.transport).await
}

pub fn routes_with_db(db: Arc<Database>) -> Router {
    let cors = CorsLayer::permissive();
    
    // Initialize WebSocket state
    let ws_state = Arc::new(WebSocketState::new());
    
    // Get DeepSeek API key from environment
    let api_key = env::var("DEEPSEEK_API_KEY").unwrap_or_else(|_| {
        warn!("DEEPSEEK_API_KEY not found in environment, using empty string");
        String::new()
    });
    
    // Initialize chat handler
    let chat_handler = Arc::new(ChatHandler::new(
        ws_state.clone(),
        Arc::new(DeepSeekService::new(api_key)),
        Arc::new(ToolExecutorFactory::new()),
    ));

    // Initialize WebSocket transport
    let transport = Arc::new(WebSocketTransport::new(
        ws_state.clone(),
        chat_handler.clone(),
    ));

    // Create app state
    let app_state = AppState {
        ws_state: ws_state.clone(),
        transport: transport.clone(),
    };

    // Initialize Nostr components
    let (event_tx, _) = broadcast::channel(1024);
    let relay_state = Arc::new(RelayState::new(event_tx, db));

    Router::new()
        .route("/ws", axum::routing::get(ws_route))
        .merge(chat::chat_routes().with_state(chat_handler))
        .with_state(app_state)
        .with_state(relay_state)
        .layer(cors)
}