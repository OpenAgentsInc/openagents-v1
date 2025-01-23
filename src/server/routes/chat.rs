use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::server::ws::handlers::chat::ChatHandler;

pub fn chat_routes() -> Router<Arc<ChatHandler>> {
    Router::new()
        .route("/chat/{id}", get(chat_session))
        .route("/chat/tools/toggle", post(toggle_tool))
}

#[derive(Debug, Deserialize)]
pub struct ToolToggle {
    #[serde(rename = "tool-name")]
    tool: String,
}

async fn chat_session(Path(_id): Path<Uuid>) -> Response {
    // Session ID is currently unused but will be needed for session management
    StatusCode::OK.into_response()
}

async fn toggle_tool(
    State(handler): State<Arc<ChatHandler>>,
    Form(form): Form<ToolToggle>,
) -> Response {
    // Extract tool name from the form field (e.g., "tool-view_file" -> "view_file")
    let tool_name = form.tool.strip_prefix("tool-").unwrap_or(&form.tool);
    
    // For now, we'll just return OK
    StatusCode::OK.into_response()
}