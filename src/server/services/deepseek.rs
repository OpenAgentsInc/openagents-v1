use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::info;

#[derive(Debug, Clone)]
pub struct DeepSeekService {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Serialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Default)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    temperature: f32,
    max_tokens: Option<i32>,
    tools: Option<Vec<Tool>>,
    tool_choice: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Serialize)]
struct ToolFunction {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct Tool {
    #[serde(rename = "type")]
    tool_type: String,
    function: ToolFunction,
}

#[derive(Debug, Deserialize)]
struct ToolCall {
    id: String,
    #[serde(rename = "type")]
    tool_type: String,
    function: ToolCallFunction,
}

#[derive(Debug, Deserialize)]
struct ToolCallFunction {
    name: String,
    arguments: String,
}

// Streaming response types
#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
    finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
    reasoning_content: Option<String>,
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Clone)]
pub enum StreamUpdate {
    Content(String),
    Reasoning(String),
    Done,
}

// Simple calculator tool implementation
fn calculate(expression: &str) -> Result<f64> {
    // Very basic calculator that only handles addition
    let parts: Vec<&str> = expression.split('+').collect();
    let sum: f64 = parts
        .iter()
        .filter_map(|n| n.trim().parse::<f64>().ok())
        .sum();
    Ok(sum)
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

    pub async fn chat(
        &self,
        prompt: String,
        use_reasoner: bool,
    ) -> Result<(String, Option<String>)> {
        self.chat_internal(prompt, use_reasoner, false).await
    }

    pub async fn chat_stream_with_tools(
        &self,
        prompt: String,
        use_reasoner: bool,
    ) -> mpsc::Receiver<StreamUpdate> {
        let (tx, rx) = mpsc::channel(100);
        let client = self.client.clone();
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();

        // Define calculator tool
        let calculator_tool = Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "calculate".to_string(),
                description: "Calculate a mathematical expression. Currently only supports addition.".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "expression": {
                            "type": "string",
                            "description": "The mathematical expression to evaluate"
                        }
                    },
                    "required": ["expression"]
                }),
            },
        };

        tokio::spawn(async move {
            let messages = vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant that can perform calculations. When asked to calculate something, use the calculate function.".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: prompt,
                },
            ];

            let request = ChatRequest {
                model: "deepseek-chat".to_string(), // Force using chat model for function calling
                messages,
                stream: true,
                temperature: 0.7,
                tools: Some(vec![calculator_tool]),
                tool_choice: Some("auto".to_string()),
                max_tokens: None,
            };

            info!("Sending request: {:?}", request);

            let url = format!("{}/chat/completions", base_url);
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .send()
                .await;

            match response {
                Ok(response) => {
                    let mut stream = response.bytes_stream();
                    let mut buffer = String::new();

                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(chunk) => {
                                let chunk_str = String::from_utf8_lossy(&chunk);
                                buffer.push_str(&chunk_str);

                                while let Some(pos) = buffer.find('\n') {
                                    let line = buffer[..pos].trim().to_string();
                                    buffer = buffer[pos + 1..].to_string();

                                    if let Some(data) = line.strip_prefix("data: ") {
                                        if data == "[DONE]" {
                                            let _ = tx.send(StreamUpdate::Done).await;
                                            break;
                                        }

                                        info!("Received data: {}", data);
                                        match serde_json::from_str::<StreamResponse>(data) {
                                            Ok(response) => {
                                                info!("Parsed response: {:?}", response);
                                            if let Some(choice) = response.choices.first() {
                                                if let Some(ref content) = choice.delta.content {
                                                    let _ = tx.send(StreamUpdate::Content(content.to_string())).await;
                                                }
                                                if let Some(ref tool_calls) = choice.delta.tool_calls {
                                                    info!("Received tool calls: {:?}", tool_calls);
                                                    for tool_call in tool_calls {
                                                        if tool_call.function.name == "calculate" {
                                                            if let Ok(args) = serde_json::from_str::<serde_json::Value>(
                                                                &tool_call.function.arguments,
                                                            ) {
                                                                if let Some(expr) = args.get("expression") {
                                                                    if let Some(expr_str) = expr.as_str() {
                                                                        if let Ok(result) = calculate(expr_str) {
                                                                            let _ = tx
                                                                                .send(StreamUpdate::Content(
                                                                                    format!("Result: {}", result),
                                                                                ))
                                                                                .await;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                if let Some(ref reasoning) = choice.delta.reasoning_content {
                                                    let _ = tx.send(StreamUpdate::Reasoning(reasoning.to_string())).await;
                                                }
                                            }
                                            Err(e) => {
                                                info!("Failed to parse response: {}", e);
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                info!("Stream error: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("Request error: {}", e);
                }
            }
        });

        rx
    }

    pub async fn chat_stream(
        &self,
        prompt: String,
        use_reasoner: bool,
    ) -> mpsc::Receiver<StreamUpdate> {
        let (tx, rx) = mpsc::channel(100);
        let client = self.client.clone();
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();

        tokio::spawn(async move {
            let model = if use_reasoner {
                "deepseek-reasoner"
            } else {
                "deepseek-chat"
            };

            let messages = vec![ChatMessage {
                role: "user".to_string(),
                content: prompt,
            }];

            let request = ChatRequest {
                model: model.to_string(),
                messages,
                stream: true,
                temperature: 0.7,
                max_tokens: None,
                tools: None,
            };

            let url = format!("{}/chat/completions", base_url);
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .send()
                .await;

            match response {
                Ok(response) => {
                    let mut stream = response.bytes_stream();
                    let mut buffer = String::new();

                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(chunk) => {
                                let chunk_str = String::from_utf8_lossy(&chunk);
                                buffer.push_str(&chunk_str);

                                // Process complete SSE messages
                                while let Some(pos) = buffer.find('\n') {
                                    // Extract the line and update buffer without borrowing issues
                                    let line = buffer[..pos].trim().to_string();
                                    buffer = buffer[pos + 1..].to_string();

                                    if let Some(data) = line.strip_prefix("data: ") {
                                        if data == "[DONE]" {
                                            let _ = tx.send(StreamUpdate::Done).await;
                                            break;
                                        }

                                        if let Ok(response) =
                                            serde_json::from_str::<StreamResponse>(data)
                                        {
                                            if let Some(choice) = response.choices.first() {
                                                if let Some(ref content) = choice.delta.content {
                                                    let _ = tx
                                                        .send(StreamUpdate::Content(
                                                            content.to_string(),
                                                        ))
                                                        .await;
                                                }
                                                if let Some(ref reasoning) =
                                                    choice.delta.reasoning_content
                                                {
                                                    let _ = tx
                                                        .send(StreamUpdate::Reasoning(
                                                            reasoning.to_string(),
                                                        ))
                                                        .await;
                                                }
                                                if choice.finish_reason.is_some() {
                                                    let _ = tx.send(StreamUpdate::Done).await;
                                                    break;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                info!("Stream error: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("Request error: {}", e);
                }
            }
        });

        rx
    }

    async fn chat_internal(
        &self,
        prompt: String,
        use_reasoner: bool,
        stream: bool,
    ) -> Result<(String, Option<String>)> {
        info!("Making chat request to DeepSeek API");

        let model = if use_reasoner {
            "deepseek-reasoner"
        } else {
            "deepseek-chat"
        };

        let messages = vec![ChatMessage {
            role: "user".to_string(),
            content: prompt,
        }];

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            stream,
            temperature: 0.7,
            max_tokens: None,
            tools: None,
        };

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let chat_response: ChatResponse = response.json().await?;

        if let Some(choice) = chat_response.choices.first() {
            Ok((
                choice.message.content.clone(),
                choice.message.reasoning_content.clone(),
            ))
        } else {
            Err(anyhow::anyhow!("No response from model"))
        }
    }
}
