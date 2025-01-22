use mockall::mock;

// Create mock for Tool trait
mock! {
    #[derive(Debug)]
    pub Tool {
        pub fn name(&self) -> &'static str;
        pub fn description(&self) -> &'static str;
        pub fn parameters(&self) -> serde_json::Value;
        pub async fn execute(&self, args: serde_json::Value) -> Result<String, crate::server::tools::ToolError>;
    }

    impl Clone for Tool {
        fn clone(&self) -> Self;
    }
}

// Create mock for WebSocketStateService trait
mock! {
    #[derive(Debug)]
    pub WebSocketStateService {
        pub async fn broadcast(&self, msg: crate::server::ws::types::Message);
    }

    impl Clone for WebSocketStateService {
        fn clone(&self) -> Self;
    }
}

// Create mock for DeepSeekService trait
mock! {
    #[derive(Debug)]
    pub DeepSeekService {
        pub async fn chat_stream(&self, content: String, tools: Vec<serde_json::Value>) -> tokio::sync::mpsc::Receiver<crate::server::services::StreamUpdate>;
    }

    impl Clone for DeepSeekService {
        fn clone(&self) -> Self;
    }
}

// Create mock for ToolExecutorFactory trait
mock! {
    #[derive(Debug)]
    pub ToolExecutorFactory {
        pub fn create_executor(&self, tool_name: &str) -> Option<std::sync::Arc<dyn crate::server::tools::Tool>>;
        pub fn list_tools(&self) -> Vec<String>;
    }

    impl Clone for ToolExecutorFactory {
        fn clone(&self) -> Self;
    }
}

// Create mock for ChatHandlerService trait
mock! {
    #[derive(Debug)]
    pub ChatHandlerService {
        pub async fn enable_tool(&self, tool: &str) -> Result<(), crate::server::tools::ToolError>;
        pub async fn disable_tool(&self, tool: &str) -> Result<(), crate::server::tools::ToolError>;
        pub async fn handle_message(&self, msg: crate::server::ws::types::Message) -> Result<(), crate::server::tools::ToolError>;
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