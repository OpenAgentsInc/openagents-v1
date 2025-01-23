use std::sync::Arc;
use axum::{
    extract::ws::{Message as WsMessage, WebSocket, WebSocketUpgrade},
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc;
use tracing::{error, info};
use uuid::Uuid;

use super::{
    transport::WebSocketTransport,
    types::WebSocketState,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    ws_state: Arc<WebSocketState>,
    transport: Arc<WebSocketTransport>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, ws_state, transport))
}

async fn handle_socket(socket: WebSocket, ws_state: Arc<WebSocketState>, transport: Arc<WebSocketTransport>) {
    let (mut sender, mut receiver) = socket.split();

    // Generate a unique connection ID
    let conn_id = Uuid::new_v4().to_string();
    info!("New WebSocket connection: {}", conn_id);

    // Create a channel for sending messages to the WebSocket
    let (tx, mut rx) = mpsc::unbounded_channel();

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
                    if let Err(e) = transport.handle_message(&text, &recv_conn_id).await {
                        error!("Error handling message: {}", e);
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