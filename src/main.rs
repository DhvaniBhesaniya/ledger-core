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

use utils::{db,db::DbPool};
use middleware::rate_limit::RateLimiter;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: Arc<DbPool>,
    pub rate_limiter: Arc<RateLimiter>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_pool = db::create_pool(&database_url)?;

    let state = Arc::new(AppState {
        db_pool: Arc::new(db_pool),
        rate_limiter: Arc::new(RateLimiter::new()),
    });

    let cors = middleware::cors::create_cors_layer();
    let app = routes::create_router(state).layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
