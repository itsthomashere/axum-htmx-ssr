use askama::Template;
use askama_axum::IntoResponse;
use axum::{response::Html, routing::get, Router};
use tower_http::services::ServeDir;

pub fn web_routes() -> Router {
    Router::new()
        .route("/", get(login_page))
        .nest_service("/assets", ServeDir::new("assets"))
        .fallback(fallback_not_found)
}

async fn login_page() -> impl IntoResponse {
    LoginPage {}
}

async fn fallback_not_found() -> impl IntoResponse {
    Html(include_str!("../../assets/404.html"))
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginPage {}
