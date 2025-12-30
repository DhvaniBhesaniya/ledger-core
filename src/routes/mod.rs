use crate::middleware::api_key_auth::api_key_auth_middleware;
use crate::{AppState, handlers};
use axum::{
    Router, middleware as axum_middleware,
    routing::{delete, get, post},
};
use std::sync::Arc;

pub fn create_router(state: Arc<AppState>) -> Router {
    let protected_routes = Router::new()
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
        .route(
            "/api/accounts/:id/keys",
            get(handlers::api_key_handlers::get_api_keys),
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
            delete(handlers::webhook_handlers::delete_webhook),
        )
        // Admin
        .route(
            "/api/keys_list",
            get(handlers::api_key_handlers::get_all_api_keys),
        )
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            api_key_auth_middleware,
        ));

    Router::new()
        // Health check - Public
        .route("/health", get(handlers::health::health))
        // API Keys
        .route("/api/key_generate", post(handlers::api_key_handlers::generate_key))
        .merge(protected_routes)
        .with_state(state)
}
