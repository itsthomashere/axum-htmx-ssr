use shared::config::Environment;
use tracing::info;

mod public;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    let env = Environment::config()?;
    let addr = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", env.port)).await?;

    let public_route = public::routes::web_routes();

    info!("Starting server on: {}", env.port);
    axum::serve(addr, public_route).await?;
    Ok(())
}
