mod db;
mod error;
mod handlers;
mod middleware;
mod models;
mod repositories;
mod schema;
mod services;
mod utils;

use axum::{
    Router, middleware as axum_middleware,
    routing::{get, post},
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

use db::DbPool;
use middleware::{api_key_auth::api_key_auth_middleware, rate_limit::RateLimiter};

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

    let cors = CorsLayer::permissive();

    let app = Router::new()
        // Health check
        .route("/health", get(handlers::health::health))
        // Accounts
        .route(
            "/api/accounts",
            post(handlers::account_handlers::create_account),
        )
        .route(
            "/api/accounts/:id",
            get(handlers::account_handlers::get_account),
        )
        .route(
            "/api/accounts/:id/balance",
            get(handlers::account_handlers::get_balance),
        )
        // Transactions
        .route(
            "/api/transactions",
            post(handlers::transaction_handlers::create_transaction),
        )
        .route(
            "/api/transactions/:id",
            get(handlers::transaction_handlers::get_transaction),
        )
        // API Keys
        .route("/api/keys", post(handlers::api_key_handlers::generate_key))
        // Webhooks
        .route(
            "/api/webhooks",
            post(handlers::webhook_handlers::register_webhook),
        )
        .route(
            "/api/webhooks/:id",
            get(handlers::webhook_handlers::get_webhook),
        )
        .route(
            "/api/webhooks/:id",
            axum::routing::delete(handlers::webhook_handlers::delete_webhook),
        )
        // Middleware
        .layer(cors)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            api_key_auth_middleware,
        ))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("Server running on http://0.0.0.0:8080");

    axum::serve(listener, app).await?;

    Ok(())
}
