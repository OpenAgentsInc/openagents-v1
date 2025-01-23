use std::sync::Arc;
use crate::server::ws::{
    handlers::chat::ChatHandlerService,
    types::{WebSocketState, Message},
};
use serde_json::Value;
use tracing::error;

pub struct WebSocketTransport {
    ws_state: Arc<WebSocketState>,
    chat_handler: Arc<dyn ChatHandlerService>,
}

impl WebSocketTransport {
    pub fn new(ws_state: Arc<WebSocketState>, chat_handler: Arc<dyn ChatHandlerService>) -> Self {
        Self {
            ws_state,
            chat_handler,
        }
    }

    pub fn chat_handler(&self) -> &Arc<dyn ChatHandlerService> {
        &self.chat_handler
    }

    pub async fn handle_message(&self, msg_str: &str, conn_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Parse the message
        let msg_value: Value = serde_json::from_str(msg_str)?;
        
        // Extract content from the message
        let content = msg_value.get("content")
            .and_then(|v| v.as_str())
            .ok_or("Missing content field")?;

        // Create a chat message
        let msg = Message::Chat {
            content: content.to_string(),
        };

        // Handle the message with connection ID
        self.chat_handler.handle_message(msg, conn_id.to_string()).await
            .map_err(|e| {
                error!("Error handling message: {}", e);
                e.into()
            })
    }
}