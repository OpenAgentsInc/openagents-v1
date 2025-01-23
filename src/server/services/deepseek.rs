use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;
use tokio::sync::mpsc;
use tracing::{error, info};

#[derive(Debug)]
pub enum StreamUpdate {
    Content(String),
    Reasoning(String),
    Done,
}

pub struct DeepSeekService {
    api_key: String,
    client: Client,
}

#[async_trait]
pub trait DeepSeekServiceTrait: Send + Sync {
    async fn chat_stream(&self, content: String, use_tools: bool) -> mpsc::Receiver<StreamUpdate>;
}

impl DeepSeekService {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait]
impl DeepSeekServiceTrait for DeepSeekService {
    async fn chat_stream(&self, content: String, _use_tools: bool) -> mpsc::Receiver<StreamUpdate> {
        let (tx, rx) = mpsc::channel(32);
        let api_key = self.api_key.clone();
        let client = self.client.clone();

        tokio::spawn(async move {
            let request = json!({
                "model": "deepseek-chat",
                "messages": [{"role": "user", "content": content}],
                "stream": true,
            });

            let response = match client
                .post("https://api.deepseek.com/v1/chat/completions")
                .header("Authorization", format!("Bearer {}", api_key))
                .json(&request)
                .send()
                .await
            {
                Ok(res) => res,
                Err(e) => {
                    error!("API request failed: {}", e);
                    let _ = tx.send(StreamUpdate::Content(format!("Error: {}", e))).await;
                    let _ = tx.send(StreamUpdate::Done).await;
                    return;
                }
            };

            if !response.status().is_success() {
                let error_msg = format!(
                    "API error: {} - {}",
                    response.status(),
                    response.text().await.unwrap_or_default()
                );
                error!("{}", error_msg);
                let _ = tx.send(StreamUpdate::Content(format!("Error: {}", error_msg))).await;
                let _ = tx.send(StreamUpdate::Done).await;
                return;
            }

            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line["data: ".len()..];
                                if data == "[DONE]" {
                                    let _ = tx.send(StreamUpdate::Done).await;
                                    break;
                                }
                                if let Ok(response) = serde_json::from_str::<StreamResponse>(data) {
                                    if let Some(content) = response.choices[0].delta.content {
                                        let _ = tx.send(StreamUpdate::Content(content)).await;
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!("Stream error: {}", e);
                        let _ = tx.send(StreamUpdate::Content(format!("Error: {}", e))).await;
                        let _ = tx.send(StreamUpdate::Done).await;
                        break;
                    }
                }
            }
        });

        rx
    }
}

#[derive(Debug, Deserialize)]
struct StreamResponse {
    choices: Vec<StreamChoice>,
}

#[derive(Debug, Deserialize)]
struct StreamChoice {
    delta: StreamDelta,
}

#[derive(Debug, Deserialize)]
struct StreamDelta {
    content: Option<String>,
}