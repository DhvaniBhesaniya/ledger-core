use crate::{AppState, middleware::ApiKeyAuth, models::*, services, utils::app_error::AppError};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    // Extension(_auth): Extension<ApiKeyAuth>,
    Json(req): Json<CreateAccountRequest>,
) -> Result<Json<AccountCreationResponse>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::account_service::create_account(req, &mut conn)?;
    Ok(Json(response))
}

pub async fn get_account(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<AccountResponse>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::account_service::get_account(id, &mut conn)?;
    Ok(Json(response))
}

pub async fn get_balance(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let balance = services::account_service::get_balance(id, &mut conn)?;
    Ok(Json(serde_json::json!({ "balance": balance })))
}
