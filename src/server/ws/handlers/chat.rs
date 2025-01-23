use std::sync::Arc;
use serde_json::Value;
use tokio::sync::mpsc;
use mockall::automock;
use async_trait::async_trait;
use tracing::{error, info};
use crate::server::tools::{Tool, ToolError};
use crate::server::services::StreamUpdate;
use crate::server::ws::types::Message;

#[automock]
#[async_trait]
pub trait DeepSeekService: Send + Sync {
    async fn chat_stream(&self, content: String, tools: Vec<Value>) -> mpsc::Receiver<StreamUpdate>;
}

#[automock]
pub trait ToolExecutorFactory: Send + Sync {
    fn create_executor(&self, tool_name: &str) -> Option<Arc<dyn Tool>>;
    fn list_tools(&self) -> Vec<String>;
}

#[automock]
#[async_trait]
pub trait WebSocketStateService: Send + Sync {
    async fn broadcast(&self, msg: Message);
}

#[automock]
#[async_trait]
pub trait ChatHandlerService: Send + Sync {
    async fn enable_tool(&self, tool: &str) -> Result<(), ToolError>;
    async fn disable_tool(&self, tool: &str) -> Result<(), ToolError>;
    async fn handle_message(&self, msg: Message) -> Result<(), ToolError>;
}

pub struct ChatHandler {
    ws_state: Arc<dyn WebSocketStateService>,
    deepseek_service: Arc<dyn DeepSeekService>,
    tool_factory: Arc<dyn ToolExecutorFactory>,
}

impl ChatHandler {
    pub fn new(
        ws_state: Arc<dyn WebSocketStateService>,
        deepseek_service: Arc<dyn DeepSeekService>,
        tool_factory: Arc<dyn ToolExecutorFactory>,
    ) -> Self {
        Self {
            ws_state,
            deepseek_service,
            tool_factory,
        }
    }
}

#[async_trait]
impl ChatHandlerService for ChatHandler {
    async fn enable_tool(&self, _tool: &str) -> Result<(), ToolError> {
        // Implementation will be added later
        Ok(())
    }

    async fn disable_tool(&self, _tool: &str) -> Result<(), ToolError> {
        // Implementation will be added later
        Ok(())
    }

    async fn handle_message(&self, msg: Message) -> Result<(), ToolError> {
        match msg {
            Message::Chat { content } => {
                self.handle_chat(content).await?;
            }
            Message::Tool { name, arguments } => {
                self.handle_tool_call(name, arguments).await?;
            }
        }
        Ok(())
    }
}

impl ChatHandler {
    async fn handle_chat(&self, content: String) -> Result<(), ToolError> {
        info!("Handling chat message: {}", content);
        
        let tools = self.tool_factory.list_tools()
            .into_iter()
            .filter_map(|name| {
                self.tool_factory.create_executor(&name).map(|tool| {
                    serde_json::json!({
                        "name": tool.name(),
                        "description": tool.description(),
                        "parameters": tool.parameters()
                    })
                })
            })
            .collect();

        let mut stream = self.deepseek_service.chat_stream(content, tools).await;

        while let Some(update) = stream.recv().await {
            match update {
                StreamUpdate::Content(content) => {
                    info!("Broadcasting content: {}", content);
                    self.ws_state.broadcast(Message::Chat { content }).await;
                }
                StreamUpdate::Reasoning(content) => {
                    info!("Broadcasting reasoning: {}", content);
                    self.ws_state.broadcast(Message::Chat { content }).await;
                }
                StreamUpdate::ToolCall(name, arguments) => {
                    info!("Handling tool call: {} with args: {:?}", name, arguments);
                    self.handle_tool_call(name, arguments).await?;
                }
                StreamUpdate::Done => {
                    info!("Chat stream completed");
                    break;
                }
            }
        }

        Ok(())
    }

    async fn handle_tool_call(&self, name: String, arguments: Value) -> Result<(), ToolError> {
        info!("Executing tool: {} with args: {:?}", name, arguments);
        
        if let Some(tool) = self.tool_factory.create_executor(&name) {
            let result = tool.execute(arguments).await?;
            info!("Tool execution result: {}", result);
            self.ws_state.broadcast(Message::Chat { content: result }).await;
            Ok(())
        } else {
            error!("Unknown tool: {}", name);
            Err(ToolError::InvalidArguments(format!("Unknown tool: {}", name)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::tools::MockTool;
    use serde_json::json;

    #[tokio::test]
    async fn test_handle_chat() {
        let mut mock_ws = MockWebSocketStateService::new();
        let mut mock_deepseek = MockDeepSeekService::new();
        let mut mock_factory = MockToolExecutorFactory::new();

        // Set up mock_factory expectations
        mock_factory
            .expect_list_tools()
            .returning(|| vec!["test_tool".to_string()]);

        let mut mock_tool = MockTool::new();
        mock_tool
            .expect_name()
            .returning(|| "test_tool");
        mock_tool
            .expect_description()
            .returning(|| "Test tool description");
        mock_tool
            .expect_parameters()
            .returning(|| json!({}));

        let mock_tool = Arc::new(mock_tool);
        mock_factory
            .expect_create_executor()
            .returning(move |_| Some(mock_tool.clone()));

        // Set up mock_ws expectations
        mock_ws
            .expect_broadcast()
            .withf(|msg| matches!(msg, Message::Chat { content } if content == "test response"))
            .returning(|_| ());

        // Set up mock_deepseek expectations
        mock_deepseek
            .expect_chat_stream()
            .returning(move |_, _| {
                let (new_tx, new_rx) = mpsc::channel(32);
                tokio::spawn(async move {
                    let _ = new_tx.send(StreamUpdate::Content("test response".to_string())).await;
                    let _ = new_tx.send(StreamUpdate::Done).await;
                });
                new_rx
            });

        let handler = ChatHandler::new(
            Arc::new(mock_ws),
            Arc::new(mock_deepseek),
            Arc::new(mock_factory),
        );

        let result = handler.handle_message(Message::Chat {
            content: "test message".to_string(),
        }).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_tool_call() {
        let mut mock_ws = MockWebSocketStateService::new();
        let mock_deepseek = Arc::new(MockDeepSeekService::new());
        let mut mock_factory = MockToolExecutorFactory::new();
        let mut mock_tool = MockTool::new();

        // Set up mock_tool expectations
        mock_tool
            .expect_execute()
            .returning(|_| Ok("tool result".to_string()));

        mock_tool
            .expect_name()
            .returning(|| "test_tool");

        mock_tool
            .expect_description()
            .returning(|| "Test tool description");

        mock_tool
            .expect_parameters()
            .returning(|| json!({}));

        // Set up mock_ws expectations
        mock_ws
            .expect_broadcast()
            .withf(|msg| matches!(msg, Message::Chat { content } if content == "tool result"))
            .returning(|_| ());

        let mock_tool = Arc::new(mock_tool);
        mock_factory
            .expect_create_executor()
            .returning(move |_| Some(mock_tool.clone()));

        let handler = ChatHandler::new(
            Arc::new(mock_ws),
            mock_deepseek,
            Arc::new(mock_factory),
        );

        let result = handler.handle_message(Message::Tool {
            name: "test_tool".to_string(),
            arguments: json!({"arg": "value"}),
        }).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_handle_unknown_tool() {
        let mock_ws = Arc::new(MockWebSocketStateService::new());
        let mock_deepseek = Arc::new(MockDeepSeekService::new());
        let mut mock_factory = MockToolExecutorFactory::new();

        mock_factory
            .expect_create_executor()
            .returning(|_| None);

        let handler = ChatHandler::new(
            mock_ws,
            mock_deepseek,
            Arc::new(mock_factory),
        );

        let result = handler.handle_message(Message::Tool {
            name: "unknown_tool".to_string(),
            arguments: json!({}),
        }).await;

        assert!(matches!(result, Err(ToolError::InvalidArguments(_))));
    }
}