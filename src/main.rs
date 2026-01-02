mod handlers;
mod middleware;
mod models;
mod repositories;
mod routes;
mod schema;
mod services;
mod utils;

use std::sync::Arc;
use tracing_subscriber;

use middleware::rate_limit::RateLimiter;
use utils::{db, db::DbPool};

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub rate_limiter: Arc<RateLimiter>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    // Enhanced logging configuration
    tracing_subscriber::fmt()
        .with_target(false) // Don't show module paths
        .with_thread_ids(false)
        .with_level(true)
        .with_ansi(true) // Enable colors
        .with_file(false)
        .with_line_number(false)
        .compact()
        .init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = db::create_pool(&database_url)?;

    let state = Arc::new(AppState {
        db_pool: Arc::new(db_pool),
        rate_limiter: Arc::new(RateLimiter::new()),
    });

    let cors = middleware::cors::create_cors_layer();
    let app = routes::create_router(state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;

    tracing::info!("╔════════════════════════════════════════╗");
    tracing::info!("║   Ledger Core API Server Started       ║");
    tracing::info!("╠════════════════════════════════════════╣");
    tracing::info!("║  Address: http://0.0.0.0:8080          ║");
    tracing::info!(
        "║  Environment: {}              ║",
        std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string())
    );
    tracing::info!("╚════════════════════════════════════════╝");

    axum::serve(listener, app).await?;

    Ok(())
}
