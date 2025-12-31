use crate::{AppState, repositories, utils::app_error::AppError};
use axum::{
    extract::{Request, State},
    http::Method,
    middleware::Next,
    response::Response,
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct ApiKeyAuth {
    pub account_id: i64,
    // pub api_key_id: i64,
    // pub rate_limit: i32,
}

pub async fn api_key_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let path_vec = ["/api/accounts", "/api/keys_list","/api/keys/"];
    if path_vec.contains(&req.uri().path()) && (req.method() == Method::POST || req.method() == Method::GET) {
        return Ok(next.run(req).await);
    }

    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::InvalidApiKey)?
        .to_string();

    let key_hash = hash_api_key(&api_key);

    let mut conn = state
        .db_pool
        .get()
        .map_err(|_| AppError::InternalError("DB connection failed".to_string()))?;

    let api_key_record = repositories::get_api_key_by_hash(&key_hash, &mut conn)?;

    if state
        .rate_limiter
        .check_limit(api_key_record.id, api_key_record.rate_limit_per_minute)
        .is_err()
    {
        return Err(AppError::RateLimitExceeded);
    }

    repositories::update_last_used(api_key_record.id, &mut conn).ok();

    let auth = ApiKeyAuth {
        account_id: api_key_record.account_id,
        // api_key_id: api_key_record.id,
        // rate_limit: api_key_record.rate_limit_per_minute,
    };

    req.extensions_mut().insert(auth);
    Ok(next.run(req).await)
}

fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key);
    format!("{:x}", hasher.finalize())
}
