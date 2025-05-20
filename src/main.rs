use anyhow::{Context, Result};
use axum::{Router, response::Json, routing::get};
use dotenvy::dotenv;
use serde_json::{Value, json};
use std::{env, net::SocketAddr};
use tracing::info;

mod db;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let pool = db::pool::create_pool().await?;
    db::pool::db_health_check(&pool).await?;

    let port = env::var("PORT").unwrap_or("3000".to_string());
    let addr = format!("0.0.0.0:{}", port)
        .parse::<SocketAddr>()
        .context("Invalid Address")?;

    let app = Router::new().route("/hello", get(hello));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .context("Failed to bind to port")?;

    info!("🚀 Started @ http://{}", addr);
    axum::serve(listener, app).await.context("Server Failed")?;

    Ok(())
}

async fn hello() -> Json<Value> {
    Json(json!({"message": "Hello There!"}))
}
