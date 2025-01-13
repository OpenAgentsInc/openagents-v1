use actix_web::{get, web, HttpResponse, Result};
use askama::Template;

use crate::emailoptin::subscribe;

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

#[get("/")]
pub async fn index_page() -> Result<HttpResponse> {
    let template = HomeTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/home")]
pub async fn home_page() -> Result<HttpResponse> {
    let template = HomeTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/health")]
pub async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy"
    })))
}

#[get("/agents")]
pub async fn agents_page() -> Result<HttpResponse> {
    let template = AgentsTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/video-series")]
pub async fn video_series_page() -> Result<HttpResponse> {
    let template = VideoSeriesTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/changelog")]
pub async fn changelog_page() -> Result<HttpResponse> {
    let template = ChangelogTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/business")]
pub async fn business_page() -> Result<HttpResponse> {
    let template = BusinessTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/contact")]
pub async fn contact_page() -> Result<HttpResponse> {
    let template = ContactTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(index_page)
        .service(home_page)
        .service(health_check)
        .service(agents_page)
        .service(video_series_page)
        .service(changelog_page)
        .service(business_page)
        .service(contact_page)
        .route("/subscriptions", web::post().to(subscribe));
}