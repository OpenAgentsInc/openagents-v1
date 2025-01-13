use askama::Template;
use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Template)]
#[template(path = "hello.html")]
struct HelloTemplate {
    title: String,
}

async fn handler() -> Html<String> {
    let template = HelloTemplate {
        title: "Hello, World!".to_string(),
    };
    Html(template.render().unwrap())
}