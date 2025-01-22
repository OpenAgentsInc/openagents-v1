use mockall::predicate::*;
use mockall::mock;
use async_trait::async_trait;

// Re-export mockall predicates
pub use mockall::predicate::*;

// Create mock for Tool trait
mock! {
    #[derive(Debug)]
    pub Tool {
        fn name(&self) -> &'static str;
        fn description(&self) -> &'static str;
        fn parameters(&self) -> serde_json::Value;
        async fn execute(&self, args: serde_json::Value) -> Result<String, crate::server::tools::ToolError>;
    }

    impl Clone for Tool {
        fn clone(&self) -> Self;
    }
}

// Create mock for WebSocketStateService trait
mock! {
    #[derive(Debug)]
    pub WebSocketStateService {
        async fn broadcast(&self, msg: crate::server::ws::types::Message);
    }

    impl Clone for WebSocketStateService {
        fn clone(&self) -> Self;
    }
}

// Create mock for DeepSeekService trait
mock! {
    #[derive(Debug)]
    pub DeepSeekService {
        async fn chat_stream(&self, content: String, tools: Vec<serde_json::Value>) -> tokio::sync::mpsc::Receiver<crate::server::services::StreamUpdate>;
    }

    impl Clone for DeepSeekService {
        fn clone(&self) -> Self;
    }
}

// Create mock for ToolExecutorFactory trait
mock! {
    #[derive(Debug)]
    pub ToolExecutorFactory {
        fn create_executor(&self, tool_name: &str) -> Option<std::sync::Arc<dyn crate::server::tools::Tool>>;
        fn list_tools(&self) -> Vec<String>;
    }

    impl Clone for ToolExecutorFactory {
        fn clone(&self) -> Self;
    }
}

// Create mock for ChatHandlerService trait
mock! {
    #[derive(Debug)]
    pub ChatHandlerService {
        async fn enable_tool(&self, tool: &str) -> Result<(), crate::server::tools::ToolError>;
        async fn disable_tool(&self, tool: &str) -> Result<(), crate::server::tools::ToolError>;
        async fn handle_message(&self, msg: crate::server::ws::types::Message) -> Result<(), crate::server::tools::ToolError>;
    }

    impl Clone for ChatHandlerService {
        fn clone(&self) -> Self;
    }
}

// Implement Send + Sync for all mock types
unsafe impl Send for MockTool {}
unsafe impl Sync for MockTool {}
unsafe impl Send for MockWebSocketStateService {}
unsafe impl Sync for MockWebSocketStateService {}
unsafe impl Send for MockDeepSeekService {}
unsafe impl Sync for MockDeepSeekService {}
unsafe impl Send for MockToolExecutorFactory {}
unsafe impl Sync for MockToolExecutorFactory {}
unsafe impl Send for MockChatHandlerService {}
unsafe impl Sync for MockChatHandlerService {}