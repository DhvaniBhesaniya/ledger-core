use crate::{
    AppState,
    middleware::{ApiKeyAuth, authorization},
    models::*,
    services,
    utils::app_error::AppError,
};
use axum::{Extension, Json, extract::State};
use std::sync::Arc;

pub async fn generate_key(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<GenerateApiKeyRequest>,
) -> Result<Json<GenerateApiKeyResponse>, AppError> {
    // Only admins can generate API keys
    authorization::require_admin(&auth)?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::generate_key(req, &mut conn)?;

    Ok(Json(response))
}

pub async fn get_api_keys(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    axum::extract::Path(account_id): axum::extract::Path<i64>,
) -> Result<Json<Vec<ApiKeyResponse>>, AppError> {
    // Require account access (admin or own account)
    authorization::require_account_access(&auth, account_id)?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::list_api_keys(account_id, &mut conn)?;
    Ok(Json(response))
}

pub async fn get_all_api_keys(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
) -> Result<Json<Vec<ApiKeyResponse>>, AppError> {
    // Only admins can list all API keys
    authorization::require_admin(&auth)?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::list_all_api_keys(&mut conn)?;
    Ok(Json(response))
}

pub async fn update_api_key(
    State(state): State<Arc<AppState>>,
    Extension(auth): Extension<ApiKeyAuth>,
    axum::extract::Path(key_id): axum::extract::Path<i64>,
    Json(req): Json<UpdateApiKeyRequest>,
) -> Result<Json<ApiKeyResponse>, AppError> {
    // Only admins can update API keys
    authorization::require_admin(&auth)?;

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::update_api_key(key_id, req, &mut conn)?;
    Ok(Json(response))
}
