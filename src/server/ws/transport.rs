use axum::extract::ws::Message;
use futures::StreamExt;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{error, info};

use super::handlers::{chat::ChatHandler, solver::SolverHandler, MessageHandler};
use super::types::ChatMessage;
use crate::server::services::DeepSeekService;

pub struct WebSocketState {
    connections: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    deepseek_service: Arc<DeepSeekService>,
}

impl WebSocketState {
    pub fn new(deepseek_service: Arc<DeepSeekService>) -> Arc<Self> {
        Arc::new(Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            deepseek_service,
        })
    }

    pub fn create_handlers(
        ws_state: Arc<WebSocketState>,
    ) -> (Arc<ChatHandler>, Arc<SolverHandler>) {
        let chat_handler = Arc::new(ChatHandler::new(
            ws_state.clone(),
            ws_state.deepseek_service.clone(),
        ));
        let solver_handler = Arc::new(SolverHandler::new());
        (chat_handler, solver_handler)
    }

    pub async fn add_connection(&self, id: String, tx: mpsc::UnboundedSender<Message>) {
        let mut conns = self.connections.write().await;
        conns.insert(id, tx);
    }

    pub async fn remove_connection(&self, id: &str) {
        let mut conns = self.connections.write().await;
        conns.remove(id);
    }

    pub async fn broadcast(&self, msg: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Broadcasting message: {}", msg);
        let conns = self.connections.read().await;
        for tx in conns.values() {
            tx.send(Message::Text(msg.to_string().into()))?;
        }
        Ok(())
    }

    pub async fn send_to(
        &self,
        conn_id: &str,
        msg: &str,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("Sending message to {}: {}", conn_id, msg);
        if let Some(tx) = self.connections.read().await.get(conn_id) {
            tx.send(Message::Text(msg.to_string().into()))?;
            info!("Message sent successfully");
        } else {
            error!("Connection {} not found", conn_id);
        }
        Ok(())
    }
}