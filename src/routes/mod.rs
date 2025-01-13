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

async fn home() -> Html<String> {
    let template = HomeTemplate {};
    Html(template.render().unwrap())
}

async fn video_series() -> Html<String> {
    let template = VideoSeriesTemplate {};
    Html(template.render().unwrap())
}

pub fn routes() -> Router {
    Router::new()
        .route("/", get(home))
        .route("/video-series", get(video_series))
}