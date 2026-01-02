use crate::{AppState, middleware::ApiKeyAuth, models::*, services, utils::app_error::AppError};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn register_webhook(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<RegisterWebhookRequest>,
) -> Result<Json<WebhookEndpointResponse>, AppError> {
    // Customer keys must have an account_id
    let account_id = auth
        .account_id
        .ok_or_else(|| AppError::BadRequest("Admin keys cannot register webhooks".to_string()))?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::webhook_service::register_webhook(account_id, req, &mut conn)?;

    Ok(Json(response))
}

pub async fn get_webhook(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<WebhookEndpointResponse>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::webhook_service::get_webhook(id, &mut conn)?;
    Ok(Json(response))
}

pub async fn delete_webhook(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    services::webhook_service::delete_webhook(id, &mut conn)?;
    Ok(())
}
