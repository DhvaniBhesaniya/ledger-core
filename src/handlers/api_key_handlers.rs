use axum::{
    extract::State,
    Extension, Json,
};
use std::sync::Arc;
use crate::{AppState, utils::app_error::AppError, models::*, middleware::ApiKeyAuth, services};

pub async fn generate_key(
    State(state): State<Arc<AppState>>,
    // Extension(auth): Extension<ApiKeyAuth>,
    Json(req): Json<GenerateApiKeyRequest>,
) -> Result<Json<GenerateApiKeyResponse>, AppError> {
    let mut conn = state.db_pool.get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let response = services::api_key_service::generate_key(
        // auth.account_id,
        req,
        &mut conn,
    )?;
    
    Ok(Json(response))
}