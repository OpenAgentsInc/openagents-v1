use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::server::services::RepomapService;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct SolverService {
    client: Client,
    repomap_service: Arc<RepomapService>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SolverResponse {
    pub solution: String,
}

impl SolverService {
    pub fn new() -> Self {
        let api_key = std::env::var("AIDER_API_KEY").unwrap_or_else(|_| "".to_string());
        Self {
            client: Client::new(),
            repomap_service: Arc::new(RepomapService::new(api_key)),
        }
    }

    pub async fn solve_issue(&self, issue_url: String) -> Result<SolverResponse> {
        info!("Processing issue: {}", issue_url);
        
        // Extract repo URL from issue URL
        // Example: https://github.com/username/repo/issues/1 -> https://github.com/username/repo
        let repo_url = issue_url
            .split("/issues/")
            .next()
            .unwrap_or(&issue_url)
            .to_string();
        
        // Generate repomap
        let repomap_response = self.repomap_service.generate_repomap(repo_url).await?;
        
        // Take first 200 characters of the repomap
        let preview = repomap_response.repo_map
            .chars()
            .take(200)
            .collect::<String>();
        
        Ok(SolverResponse {
            solution: format!("Repository Map Preview:\n\n{}", preview),
        })
    }
}
