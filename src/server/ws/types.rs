use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedSender, RwLock};
use axum::extract::ws::Message as WsMessage;
use async_trait::async_trait;
use crate::server::ws::handlers::chat::WebSocketStateService;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Message {
    #[serde(rename = "chat")]
    Chat {
        content: String,
    },
    #[serde(rename = "tool")]
    Tool {
        name: String,
        arguments: Value,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SolverMessage {
    #[serde(rename = "solve")]
    Solve {
        problem: String,
    },
    #[serde(rename = "solution")]
    Solution {
        solution: String,
    },
}

pub struct WebSocketState {
    connections: Arc<RwLock<HashMap<String, UnboundedSender<WsMessage>>>>,
}

impl WebSocketState {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_connection(&self, id: String, tx: UnboundedSender<WsMessage>) {
        let mut connections = self.connections.write().await;
        connections.insert(id, tx);
    }

    pub async fn remove_connection(&self, id: &str) {
        let mut connections = self.connections.write().await;
        connections.remove(id);
    }

    pub async fn send_to(&self, id: &str, msg: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let connections = self.connections.read().await;
        if let Some(tx) = connections.get(id) {
            tx.send(WsMessage::Text(msg.to_string().into()))?;
            Ok(())
        } else {
            Err("Connection not found".into())
        }
    }
}

#[async_trait]
impl WebSocketStateService for WebSocketState {
    async fn broadcast(&self, msg: Message) {
        let connections = self.connections.read().await;
        let msg_str = serde_json::to_string(&msg).unwrap();
        for tx in connections.values() {
            let _ = tx.send(WsMessage::Text(msg_str.clone().into()));
        }
    }

    async fn send_to(&self, id: &str, msg: Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let msg_json = match msg {
            Message::Chat { content } => {
                serde_json::json!({
                    "type": "chat",
                    "content": content,
                    "sender": "ai",
                    "status": "streaming"
                })
            },
            Message::Tool { name, arguments } => {
                serde_json::json!({
                    "type": "tool",
                    "name": name,
                    "arguments": arguments
                })
            }
        };
        let msg_str = serde_json::to_string(&msg_json)?;
        self.send_to(id, &msg_str).await
    }
}

impl Default for WebSocketState {
    fn default() -> Self {
        Self::new()
    }
}