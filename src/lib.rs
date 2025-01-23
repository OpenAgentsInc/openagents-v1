pub mod agents;
pub mod configuration;
pub mod database;
pub mod emailoptin;
pub mod nostr;
pub mod server;

pub use server::services::handle_solver;
pub use server::app;

use askama::Template;
use axum::{
    extract::{Form, State},
    http::header::{HeaderMap, HeaderValue},
    response::{Html, IntoResponse, Response},
};
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{error, info};

#[cfg(test)]
pub mod test_utils;

pub fn render_markdown(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

#[derive(Template)]
#[template(path = "layouts/base.html", escape = "none")]
pub struct PageTemplate<'a> {
    pub title: &'a str,
    pub path: &'a str,
}

#[derive(Template)]
#[template(path = "layouts/chat_base.html", escape = "none")]
pub struct ChatPageTemplate<'a> {
    pub title: &'a str,
    pub path: &'a str,
}

#[derive(Template)]
#[template(path = "layouts/content.html", escape = "none")]
pub struct ContentTemplate<'a> {
    pub path: &'a str,
}

#[derive(Template)]
#[template(path = "layouts/chat_content.html", escape = "none")]
pub struct ChatContentTemplate;

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
        let template = PageTemplate { title, path };
        Html(template.render().unwrap()).into_response()
    }
}

pub async fn solver_page(headers: HeaderMap) -> Response {
    let is_htmx = headers.contains_key("hx-request");
    let title = "Issue Solver";
    let path = "/solver";

    if is_htmx {
        let content = ContentTemplate { path }.render().unwrap();
        let mut response = Response::new(content.into());
        response.headers_mut().insert(
            "HX-Title",
            HeaderValue::from_str(&format!("OpenAgents - {}", title)).unwrap(),
        );
        response
    } else {
        let template = PageTemplate { title, path };
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
        }
        Err(e) => {
            error!("Failed to generate repomap: {}", e);
            if let Some(source) = e.source() {
                error!("Error source: {}", source);
            }
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to generate repomap: {}", e),
            )
                .into_response()
        }
    }
}