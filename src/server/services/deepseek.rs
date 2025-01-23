use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{error, info};
use async_trait::async_trait;
use serde_json::Value;
use crate::server::ws::handlers::chat::DeepSeekService as DeepSeekServiceTrait;

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

#[derive(Debug, Serialize)]
struct ResponseFormat {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    stream: bool,
    temperature: f32,
    max_tokens: i32,
    frequency_penalty: f32,
    presence_penalty: f32,
    response_format: ResponseFormat,
    stop: Option<Vec<String>>,
    stream_options: Option<Value>,
    top_p: f32,
    tools: Option<Vec<Value>>,
    tool_choice: String,
    logprobs: bool,
    top_logprobs: Option<Value>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    message: ChatResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ChatResponseMessage {
    content: String,
    role: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<ChatChoice>,
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
    role: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Clone)]
pub enum StreamUpdate {
    Content(String),
    Reasoning(String),
    ToolCall(String, Value),
    Done,
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

    async fn chat_internal(
        &self,
        prompt: String,
        _use_reasoner: bool,
        stream: bool,
    ) -> Result<(String, Option<String>)> {
        info!("Making chat request to DeepSeek API");

        let messages = vec![
            ChatMessage {
                role: "system".to_string(),
                content: "You are a helpful assistant".to_string(),
            },
            ChatMessage {
                role: "user".to_string(),
                content: prompt,
            },
        ];

        let request = ChatRequest {
            model: "deepseek-chat".to_string(),
            messages,
            stream,
            temperature: 1.0,
            max_tokens: 2048,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            response_format: ResponseFormat {
                type_: "text".to_string(),
            },
            stop: None,
            stream_options: None,
            top_p: 1.0,
            tools: None,
            tool_choice: "none".to_string(),
            logprobs: false,
            top_logprobs: None,
        };

        let url = format!("{}/chat/completions", self.base_url);
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            error!("API error: {} - {}", status, error_text);
            return Err(anyhow::anyhow!("API error: {}", error_text));
        }

        let chat_response: ChatResponse = response.json().await?;

        if let Some(choice) = chat_response.choices.first() {
            Ok((choice.message.content.clone(), None))
        } else {
            Err(anyhow::anyhow!("No response from model"))
        }
    }
}

#[async_trait]
impl DeepSeekServiceTrait for DeepSeekService {
    async fn chat_stream(&self, content: String, _tools: Vec<Value>) -> mpsc::Receiver<StreamUpdate> {
        let (tx, rx) = mpsc::channel(100);
        let client = self.client.clone();
        let api_key = self.api_key.clone();
        let base_url = self.base_url.clone();

        tokio::spawn(async move {
            let messages = vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: "You are a helpful assistant".to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content,
                },
            ];

            let request = ChatRequest {
                model: "deepseek-chat".to_string(),
                messages,
                stream: true,
                temperature: 1.0,
                max_tokens: 2048,
                frequency_penalty: 0.0,
                presence_penalty: 0.0,
                response_format: ResponseFormat {
                    type_: "text".to_string(),
                },
                stop: None,
                stream_options: None,
                top_p: 1.0,
                tools: None,
                tool_choice: "none".to_string(),
                logprobs: false,
                top_logprobs: None,
            };

            let url = format!("{}/chat/completions", base_url);
            let response = client
                .post(&url)
                .header("Content-Type", "application/json")
                .header("Accept", "application/json")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .send()
                .await;

            match response {
                Ok(response) => {
                    let status = response.status();
                    if !status.is_success() {
                        let error_text = response.text().await.unwrap_or_default();
                        error!("API error: {} - {}", status, error_text);
                        return;
                    }

                    let mut stream = response.bytes_stream();
                    let mut buffer = String::new();

                    while let Some(chunk) = stream.next().await {
                        match chunk {
                            Ok(chunk) => {
                                let chunk_str = String::from_utf8_lossy(&chunk);
                                buffer.push_str(&chunk_str);

                                // Process complete SSE messages
                                while let Some(pos) = buffer.find('\n') {
                                    let line = buffer[..pos].trim().to_string();
                                    buffer = buffer[pos + 1..].to_string();

                                    if let Some(data) = line.strip_prefix("data: ") {
                                        if data == "[DONE]" {
                                            let _ = tx.send(StreamUpdate::Done).await;
                                            break;
                                        }

                                        if let Ok(response) = serde_json::from_str::<StreamResponse>(data)
                                        {
                                            if let Some(choice) = response.choices.first() {
                                                if let Some(ref content) = choice.delta.content {
                                                    let _ = tx
                                                        .send(StreamUpdate::Content(
                                                            content.to_string(),
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
                                error!("Stream error: {}", e);
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Request error: {}", e);
                }
            }
        });

        rx
    }
}