use std::sync::Arc;
use tokio::sync::mpsc;
use crate::server::ws::handlers::chat::ChatHandlerService;
use crate::server::ws::types::{Message, WebSocketState};

pub struct WebSocketTransport {
    chat_handler: Arc<dyn ChatHandlerService>,
}

impl WebSocketTransport {
    pub fn new(ws_state: Arc<WebSocketState>, chat_handler: Arc<dyn ChatHandlerService>) -> Self {
        Self {
            chat_handler,
        }
    }

    pub async fn handle_connection(&self, _conn_id: String, mut rx: mpsc::Receiver<Message>) {
        while let Some(msg) = rx.recv().await {
            if let Err(e) = self.chat_handler.handle_message(msg).await {
                eprintln!("Error handling message: {}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::tools::ToolError;
    use crate::server::ws::handlers::chat::MockChatHandlerService;

    #[tokio::test]
    async fn test_handle_connection() {
        let ws_state = Arc::new(WebSocketState::new());
        let mut mock_handler = MockChatHandlerService::new();
        
        mock_handler
            .expect_handle_message()
            .returning(|_| Ok(()));

        let transport = WebSocketTransport::new(
            ws_state,
            Arc::new(mock_handler) as Arc<dyn ChatHandlerService>,
        );

        let (tx, rx) = mpsc::channel(32);
        tx.send(Message::Chat {
            content: "test".to_string(),
        }).await.unwrap();
        drop(tx);

        transport.handle_connection("test_conn".to_string(), rx).await;
    }

    #[tokio::test]
    async fn test_handle_connection_error() {
        let ws_state = Arc::new(WebSocketState::new());
        let mut mock_handler = MockChatHandlerService::new();
        
        mock_handler
            .expect_handle_message()
            .returning(|_| Err(ToolError::ExecutionFailed("test error".to_string())));

        let transport = WebSocketTransport::new(
            ws_state,
            Arc::new(mock_handler) as Arc<dyn ChatHandlerService>,
        );

        let (tx, rx) = mpsc::channel(32);
        tx.send(Message::Chat {
            content: "test".to_string(),
        }).await.unwrap();
        drop(tx);

        transport.handle_connection("test_conn".to_string(), rx).await;
    }
}