use crate::{AppState, middleware::ApiKeyAuth, models::*, services, utils::app_error::AppError};
use axum::{
    Extension, Json,
    extract::{Path, State},
};
use std::sync::Arc;

pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    // Customer keys must have an account_id
    let account_id = auth
        .account_id
        .ok_or_else(|| AppError::BadRequest("Admin keys cannot create transactions".to_string()))?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::transaction_service::create_transaction(account_id, req, &mut conn)?;

    Ok(Json(response))
}

pub async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<TransactionResponse>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::transaction_service::get_transaction(id, &mut conn)?;
    Ok(Json(response))
}
