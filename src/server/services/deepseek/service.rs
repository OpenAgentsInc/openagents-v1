use reqwest::Client;

use super::types::{FunctionDefinition, Tool};

#[derive(Debug, Clone)]
pub struct DeepSeekService {
    pub(crate) client: Client,
    pub(crate) api_key: String,
    pub(crate) base_url: String,
}

impl DeepSeekService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.deepseek.com".to_string(),
        }
    }

    pub fn with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub fn create_tool(
        name: String,
        description: Option<String>,
        parameters: serde_json::Value,
    ) -> Tool {
        Tool {
            tool_type: "function".to_string(),
            function: FunctionDefinition {
                name,
                description,
                parameters,
            },
        }
    }
}