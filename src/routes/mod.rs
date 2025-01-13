use axum::{
    response::Html,
    routing::get,
    Router,
};
use askama::Template;

#[derive(Template)]
#[template(path = "pages/home.html")]
struct HomeTemplate;

#[derive(Template)]
#[template(path = "pages/video-series.html")]
struct VideoSeriesTemplate;

#[derive(Template)]
#[template(path = "pages/agents.html")]
struct AgentsTemplate;

#[derive(Template)]
#[template(path = "pages/business.html")]
struct BusinessTemplate;

#[derive(Template)]
#[template(path = "pages/contact.html")]
struct ContactTemplate;

#[derive(Template)]
#[template(path = "pages/changelog.html")]
struct ChangelogTemplate;

async fn home() -> Html<String> {
    let template = HomeTemplate {};
    Html(template.render().unwrap())
}

async fn video_series() -> Html<String> {
    let template = VideoSeriesTemplate {};
    Html(template.render().unwrap())
}

async fn agents() -> Html<String> {
    let template = AgentsTemplate {};
    Html(template.render().unwrap())
}

async fn business() -> Html<String> {
    let template = BusinessTemplate {};
    Html(template.render().unwrap())
}

async fn contact() -> Html<String> {
    let template = ContactTemplate {};
    Html(template.render().unwrap())
}

async fn changelog() -> Html<String> {
    let template = ChangelogTemplate {};
    Html(template.render().unwrap())
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/video-series", get(video_series))
        .route("/agents", get(agents))
        .route("/business", get(business))
        .route("/contact", get(contact))
        .route("/changelog", get(changelog))
}