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

use crate::server::ws::handlers::chat::ChatHandlerService;

pub fn chat_routes() -> Router<Arc<dyn ChatHandlerService>> {
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
    State(handler): State<Arc<dyn ChatHandlerService>>,
    Form(form): Form<ToolToggle>,
) -> Response {
    // Extract tool name from the form field (e.g., "tool-view_file" -> "view_file")
    let tool_name = form.tool.strip_prefix("tool-").unwrap_or(&form.tool);
    
    // For now, we'll just enable tools (disable will be handled by unchecking)
    let result = handler.enable_tool(tool_name).await;

    match result {
        Ok(_) => StatusCode::OK.into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::Request;
    use axum::body::Body;
    use tower::ServiceExt;
    use crate::server::ws::handlers::chat::MockChatHandlerService;

    #[tokio::test]
    async fn test_chat_session() {
        let app = Router::new()
            .route("/chat/{id}", get(chat_session))
            .with_state(Arc::new(MockChatHandlerService::new()) as Arc<dyn ChatHandlerService>);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/chat/123e4567-e89b-12d3-a456-426614174000")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_toggle_tool() {
        let mut mock_handler = MockChatHandlerService::new();
        mock_handler
            .expect_enable_tool()
            .withf(|tool| tool == "view_file")
            .returning(|_| Ok(()));

        let app = Router::new()
            .route("/chat/tools/toggle", post(toggle_tool))
            .with_state(Arc::new(mock_handler) as Arc<dyn ChatHandlerService>);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/chat/tools/toggle")
                    .method("POST")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from("tool-name=tool-view_file"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_toggle_tool_error() {
        let mut mock_handler = MockChatHandlerService::new();
        mock_handler
            .expect_enable_tool()
            .returning(|_| Err(crate::server::tools::ToolError::InvalidArguments("test error".to_string())));

        let app = Router::new()
            .route("/chat/tools/toggle", post(toggle_tool))
            .with_state(Arc::new(mock_handler) as Arc<dyn ChatHandlerService>);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/chat/tools/toggle")
                    .method("POST")
                    .header("content-type", "application/x-www-form-urlencoded")
                    .body(Body::from("tool-name=tool-unknown"))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}