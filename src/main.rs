use askama::Template;
use axum::{response::IntoResponse, routing::get, Router};
use shared::config::Environment;
use tower_http::services::ServeDir;
use tracing::info;

mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let env = Environment::config()?;
    let addr = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", env.port)).await?;

    info!("Starting server on: {}", env.port);
    axum::serve(addr, router()).await?;
    Ok(())
}

fn router() -> Router {
    let dir = ServeDir::new("assets");
    info!("{:?}", dir);
    Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(hello_route))
}

async fn hello_route() -> impl IntoResponse {
    HelloTemplate {}
}

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate {}
