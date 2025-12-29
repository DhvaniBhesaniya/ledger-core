use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use crate::{AppState, error::AppError, models::*, middleware::ApiKeyAuth, services};

pub async fn register_webhook(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<RegisterWebhookRequest>,
) -> Result<Json<WebhookEndpointResponse>, AppError> {
    let mut conn = state.db_pool.get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::webhook_service::register_webhook(
        auth.account_id,
        req,
        &mut conn,
    )?;
    
    Ok(Json(response))
}

pub async fn get_webhook(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<WebhookEndpointResponse>, AppError> {
    let mut conn = state.db_pool.get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::webhook_service::get_webhook(id, &mut conn)?;
    Ok(Json(response))
}

pub async fn delete_webhook(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let mut conn = state.db_pool.get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    services::webhook_service::delete_webhook(id, &mut conn)?;
    Ok(())
}