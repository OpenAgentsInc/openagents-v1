pub mod agents;
pub mod configuration;
pub mod database;
pub mod emailoptin;
pub mod nostr;
pub mod server;

use askama::Template;

mod template_filters;

// Register custom filters
pub static FILTERS: askama::Filters = askama::Filters::new()
    .add("markdown", template_filters::markdown);
use axum::{
    http::header::{HeaderMap, HeaderValue},
    response::{Html, IntoResponse, Response},
    extract::{State, Form},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{info, error};

#[derive(Template)]
#[template(path = "layouts/base.html")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub path: &'a str,
}

#[derive(Template)]
#[template(path = "layouts/content.html")]
pub struct ContentTemplate<'a> {
    pub path: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct RepomapRequest {
    pub repo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepomapResponse {
    pub repo_map: String,
    pub metadata: serde_json::Value,
}

pub async fn repomap(headers: HeaderMap) -> Response {
    let is_htmx = headers.contains_key("hx-request");
    let title = "Repository Map";
    let path = "/repomap";

    if is_htmx {
        let content = ContentTemplate { path }.render().unwrap();
        let mut response = Response::new(content.into());
        response.headers_mut().insert(
            "HX-Title",
            HeaderValue::from_str(&format!("OpenAgents - {}", title)).unwrap(),
        );
        response
    } else {
        let template = PageTemplate {
            title,
            path,
        };
        Html(template.render().unwrap()).into_response()
    }
}

pub async fn generate_repomap(
    State(service): State<Arc<server::services::RepomapService>>,
    Form(req): Form<RepomapRequest>,
) -> Response {
    info!("Generating repomap for: {}", req.repo_url);
    
    match service.generate_repomap(req.repo_url).await {
        Ok(repomap) => {
            info!("Successfully generated repomap");
            Html(repomap.repo_map).into_response()
        },
        Err(e) => {
            error!("Failed to generate repomap: {}", e);
            if let Some(source) = e.source() {
                error!("Error source: {}", source);
            }
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate repomap: {}", e)
            ).into_response()
        }
    }
}
