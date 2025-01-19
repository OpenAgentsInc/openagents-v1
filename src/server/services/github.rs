use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Debug, Clone)]
pub struct GitHubService {
    client: Client,
    api_key: String,
    base_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GitHubIssue {
    pub title: String,
    pub body: String,
    pub number: i32,
    pub state: String,
}

impl GitHubService {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.github.com".to_string(),
        }
    }

    pub fn new_with_base_url(api_key: String, base_url: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url,
        }
    }

    pub async fn get_issue(&self, owner: &str, repo: &str, issue_number: i32) -> Result<GitHubIssue> {
        info!("Fetching GitHub issue: {}/{}/{}", owner, repo, issue_number);

        let url = format!(
            "{}/repos/{}/{}/issues/{}",
            self.base_url, owner, repo, issue_number
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("User-Agent", "OpenAgents")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!(
                "GitHub API error ({}): {}", 
                status,
                error_text
            ));
        }

        let mut issue: GitHubIssue = response.json().await?;
        
        // Escape HTML in title and body
        issue.title = html_escape::encode_text(&issue.title).into_owned();
        issue.body = html_escape::encode_text(&issue.body).into_owned();
        
        Ok(issue)
    }

    pub fn parse_issue_url(url: &str) -> Result<(String, String, i32)> {
        let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
        if parts.len() >= 5 && parts[parts.len() - 2] == "issues" {
            Ok((
                parts[parts.len() - 4].to_string(),
                parts[parts.len() - 3].to_string(),
                parts[parts.len() - 1].parse()?
            ))
        } else {
            Err(anyhow::anyhow!("Invalid GitHub issue URL format"))
        }
    }
}