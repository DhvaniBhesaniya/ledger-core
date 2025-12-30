use crate::{AppState, models::*, services, utils::app_error::AppError};
use axum::{Json, extract::State};
use std::sync::Arc;

pub async fn generate_key(
    State(state): State<Arc<AppState>>,
    // Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<GenerateApiKeyRequest>,
) -> Result<Json<GenerateApiKeyResponse>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::generate_key(
        // auth.account_id,
        req, &mut conn,
    )?;

    Ok(Json(response))
}

pub async fn get_api_keys(
    State(state): State<Arc<AppState>>,
    // Extension(_auth): Extension<ApiKeyAuth>,
    axum::extract::Path(account_id): axum::extract::Path<i64>,
) -> Result<Json<Vec<ApiKeyResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::list_api_keys(account_id, &mut conn)?;
    Ok(Json(response))
}

pub async fn get_all_api_keys(
    State(state): State<Arc<AppState>>,
    // Extension(_auth): Extension<ApiKeyAuth>,
) -> Result<Json<Vec<ApiKeyResponse>>, AppError> {
    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::list_all_api_keys(&mut conn)?;
    Ok(Json(response))
}
