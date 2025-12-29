use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use crate::{AppState, error::AppError, models::*, middleware::ApiKeyAuth, services};

pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<CreateTransactionRequest>,
) -> Result<Json<TransactionResponse>, AppError> {
    let mut conn = state.db_pool.get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::transaction_service::create_transaction(
        auth.account_id,
        req,
        &mut conn,
    )?;
    
    Ok(Json(response))
}

pub async fn get_transaction(
    State(state): State<Arc<AppState>>,
    Extension(_auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<TransactionResponse>, AppError> {
    let mut conn = state.db_pool.get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::transaction_service::get_transaction(id, &mut conn)?;
    Ok(Json(response))
}