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
    Extension(auth): Extension<ApiKeyAuth>,
    Path(id): Path<i64>,
) -> Result<Json<TransactionResponse>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let transaction = services::transaction_service::get_transaction(id, &mut conn)?;

    // Authorization: Admin can see any transaction, customer only their own account's transactions
    if auth.role != "admin" {
        let account_id = auth.account_id.ok_or_else(|| AppError::Forbidden)?;

        // Customer can see transaction if they are either sender or receiver
        let is_sender = transaction.from_account_id == Some(account_id);
        let is_receiver = transaction.to_account_id == Some(account_id);

        if !is_sender && !is_receiver {
            return Err(AppError::Forbidden);
        }
    }

    Ok(Json(transaction))
}

// List all transactions for an account (both sent and received)
pub async fn list_account_transactions(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    Path(account_id): Path<i64>,
) -> Result<Json<Vec<TransactionResponse>>, AppError> {
    // Require account access (admin or own account)
    if auth.role != "admin" {
        let user_account_id = auth.account_id.ok_or_else(|| AppError::Forbidden)?;

        if user_account_id != account_id {
            return Err(AppError::Forbidden);
        }
    }

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let transactions =
        services::transaction_service::list_account_transactions(account_id, &mut conn)?;

    Ok(Json(transactions))
}
