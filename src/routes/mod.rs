use actix_web::{web, HttpResponse, Result};
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

async fn home() -> Result<HttpResponse> {
    let template = HomeTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn video_series() -> Result<HttpResponse> {
    let template = VideoSeriesTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn agents() -> Result<HttpResponse> {
    let template = AgentsTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn business() -> Result<HttpResponse> {
    let template = BusinessTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn contact() -> Result<HttpResponse> {
    let template = ContactTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

async fn changelog() -> Result<HttpResponse> {
    let template = ChangelogTemplate {};
    let html = template.render().map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .route("/", web::get().to(home))
            .route("/video-series", web::get().to(video_series))
            .route("/agents", web::get().to(agents))
            .route("/business", web::get().to(business))
            .route("/contact", web::get().to(contact))
            .route("/changelog", web::get().to(changelog))
    );
}