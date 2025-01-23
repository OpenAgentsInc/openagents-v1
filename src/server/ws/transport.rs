use std::sync::Arc;
use crate::server::ws::{
    handlers::chat::ChatHandlerService,
    types::WebSocketState,
};

pub struct WebSocketTransport {
    chat_handler: Arc<dyn ChatHandlerService>,
}

impl WebSocketTransport {
    pub fn new(_ws_state: Arc<WebSocketState>, chat_handler: Arc<dyn ChatHandlerService>) -> Self {
        Self {
            chat_handler,
        }
    }

    pub fn chat_handler(&self) -> &Arc<dyn ChatHandlerService> {
        &self.chat_handler
    }
}