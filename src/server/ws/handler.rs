use std::sync::Arc;
use axum::{
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::{error, info};

use super::{
    transport::WebSocketState,
    handlers::chat::ChatHandler,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ws_state: Arc<WebSocketState>,
    chat_handler: Arc<ChatHandler>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, ws_state, chat_handler))
}

async fn handle_socket(socket: WebSocket, ws_state: Arc<WebSocketState>, chat_handler: Arc<ChatHandler>) {
    let (mut sender, mut receiver) = socket.split();

    // Generate unique connection ID
    let conn_id = uuid::Uuid::new_v4().to_string();
    info!("New WebSocket connection: {}", conn_id);

    // Create a channel for sending messages to the WebSocket
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Store the sender in WebSocketState
    ws_state.add_connection(conn_id.clone(), tx).await;

    // Clone connection ID for the receive task
    let recv_conn_id = conn_id.clone();

    // Spawn task to forward messages from rx to the WebSocket
    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if sender.send(message).await.is_err() {
                break;
            }
        }
    });

    // Handle incoming messages
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            match message {
                WsMessage::Text(text) => {
                    info!("Received message from {}: {}", recv_conn_id, text);
                    if let Ok(data) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(content) = data.get("content") {
                            if let Some(content_str) = content.as_str() {
                                let chat_msg = crate::server::ws::types::ChatMessage::UserMessage {
                                    content: content_str.to_string(),
                                };
                                if let Err(e) = chat_handler
                                    .handle_message(chat_msg, recv_conn_id.clone())
                                    .await
                                {
                                    error!("Error handling chat message: {}", e);
                                }
                            }
                        }
                    }
                }
                WsMessage::Close(_) => {
                    info!("WebSocket closed by client: {}", recv_conn_id);
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = &mut send_task => recv_task.abort(),
        _ = &mut recv_task => send_task.abort(),
    }

    // Clean up connection
    ws_state.remove_connection(&conn_id).await;
    info!("WebSocket connection closed: {}", conn_id);
}