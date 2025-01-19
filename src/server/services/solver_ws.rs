use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use bytes::Bytes;
use futures::{sink::SinkExt, stream::StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::{broadcast, mpsc};
use tracing::{error, info};

use super::solver::SolverService;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);

#[derive(Clone)]
pub struct SolverWsState {
    solver_service: Arc<SolverService>,
    connections: Arc<tokio::sync::RwLock<HashMap<String, mpsc::Sender<Message>>>>,
    update_tx: broadcast::Sender<SolverUpdate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SolverUpdate {
    Progress {
        stage: SolverStage,
        message: String,
        data: Option<serde_json::Value>,
    },
    Complete {
        result: serde_json::Value,
    },
    Error {
        message: String,
        details: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SolverStage {
    Init,
    Repomap,
    Analysis,
    Solution,
    PR,
}

impl SolverWsState {
    pub fn new(solver_service: Arc<SolverService>) -> Self {
        let (update_tx, _) = broadcast::channel(100);
        Self {
            solver_service,
            connections: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            update_tx,
        }
    }

    pub async fn broadcast_update(&self, update: SolverUpdate) {
        // Convert update to HTML fragment with hx-swap-oob
        let html = match &update {
            SolverUpdate::Progress { stage, message, data } => {
                format!(
                    r#"<div id="solver-progress" hx-swap-oob="true">
                        <div class="progress-bar" style="width: {}%">
                            {}
                        </div>
                    </div>
                    <div id="solver-status" hx-swap-oob="true">
                        Stage {}: {}
                    </div>"#,
                    match stage {
                        SolverStage::Init => 0,
                        SolverStage::Repomap => 25,
                        SolverStage::Analysis => 50,
                        SolverStage::Solution => 75,
                        SolverStage::PR => 90,
                    },
                    message,
                    match stage {
                        SolverStage::Init => "1/5",
                        SolverStage::Repomap => "2/5", 
                        SolverStage::Analysis => "3/5",
                        SolverStage::Solution => "4/5",
                        SolverStage::PR => "5/5",
                    },
                    message
                )
            }
            SolverUpdate::Complete { result } => {
                format!(
                    r#"<div id="solver-progress" hx-swap-oob="true">
                        <div class="progress-bar" style="width: 100%">
                            Complete
                        </div>
                    </div>
                    <div id="solver-status" hx-swap-oob="true">
                        Solution complete
                    </div>
                    <div id="solver-result" hx-swap-oob="true">
                        {result}
                    </div>"#
                )
            }
            SolverUpdate::Error { message, details } => {
                format!(
                    r#"<div id="solver-status" hx-swap-oob="true">
                        <div class="error">
                            Error: {message}
                            {}</div>
                    </div>"#,
                    details.as_ref().map(|d| format!("<pre>{d}</pre>")).unwrap_or_default()
                )
            }
        };

        // Send HTML update to all connected clients
        let conns = self.connections.read().await;
        for tx in conns.values() {
            let _ = tx.send(Message::Text(html.clone())).await;
        }
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<SolverWsState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<SolverWsState>) {
    let (tx, mut rx) = mpsc::channel(32);
    let (mut sender, mut receiver) = socket.split();
    let state_clone = state.clone();
    let tx_clone = tx.clone();

    // Add connection to state
    let conn_id = uuid::Uuid::new_v4().to_string();
    {
        let mut conns = state.connections.write().await;
        conns.insert(conn_id.clone(), tx.clone());
    }

    // Create broadcast subscription
    let mut update_rx = state.update_tx.subscribe();

    // Forward broadcast updates to this client
    let send_task = tokio::spawn(async move {
        let last_active = Instant::now();
        let mut heartbeat_interval = tokio::time::interval(HEARTBEAT_INTERVAL);

        loop {
            tokio::select! {
                Some(msg) = rx.recv() => {
                    if sender.send(msg).await.is_err() {
                        break;
                    }
                }
                Ok(update) = update_rx.recv() => {
                    // Convert update to HTML and send
                    let html = match update {
                        SolverUpdate::Progress { stage, message, data } => {
                            format!(
                                r#"<div id="solver-progress" hx-swap-oob="true">
                                    <div class="progress-bar" style="width: {}%">
                                        {}
                                    </div>
                                </div>"#,
                                match stage {
                                    SolverStage::Init => 0,
                                    SolverStage::Repomap => 25,
                                    SolverStage::Analysis => 50,
                                    SolverStage::Solution => 75,
                                    SolverStage::PR => 90,
                                },
                                message
                            )
                        }
                        SolverUpdate::Complete { result } => {
                            format!(
                                r#"<div id="solver-progress" hx-swap-oob="true">
                                    <div class="progress-bar" style="width: 100%">
                                        Complete
                                    </div>
                                </div>"#
                            )
                        }
                        SolverUpdate::Error { message, details } => {
                            format!(
                                r#"<div id="solver-status" hx-swap-oob="true">
                                    <div class="error">
                                        Error: {message}
                                        {}</div>
                                </div>"#,
                                details.as_ref().map(|d| format!("<pre>{d}</pre>")).unwrap_or_default()
                            )
                        }
                    };
                    if sender.send(Message::Text(html)).await.is_err() {
                        break;
                    }
                }
                _ = heartbeat_interval.tick() => {
                    if Instant::now().duration_since(last_active) > CLIENT_TIMEOUT {
                        break;
                    }
                    if sender.send(Message::Ping(Bytes::new())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Handle incoming messages
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    // Handle solver commands
                    if let Ok(cmd) = serde_json::from_str::<SolverCommand>(&text) {
                        match cmd.action.as_str() {
                            "start" => {
                                if let Some(issue_url) = cmd.issue_url {
                                    info!("Starting solver for issue: {}", issue_url);
                                    state_clone.solver_service.solve_issue_with_ws(
                                        issue_url,
                                        state_clone.update_tx.clone(),
                                    ).await;
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Message::Ping(bytes) => {
                    let _ = tx_clone.send(Message::Pong(bytes)).await;
                }
                Message::Close(_) => break,
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = send_task => {
            error!("Send task completed");
        }
        _ = recv_task => {
            error!("Receive task completed");
        }
    }

    // Remove connection from state
    let mut conns = state.connections.write().await;
    conns.remove(&conn_id);
}

#[derive(Debug, Deserialize)]
struct SolverCommand {
    action: String,
    issue_url: Option<String>,
}