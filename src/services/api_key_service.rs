use crate::{
    error::AppError,
    models::*,
    repositories,
    utils::crypto,
};
use diesel::PgConnection;
use uuid::Uuid;

pub fn generate_key(
    account_id: i64,
    _req: GenerateApiKeyRequest,
    conn: &mut PgConnection,
) -> Result<GenerateApiKeyResponse, AppError> {
    // Generate random API key
    let raw_key = format!("sk_prod_{}", Uuid::new_v4().to_string());
    let key_hash = crypto::hash_api_key(&raw_key);
    let key_prefix = raw_key[..20].to_string();

    let new_key = NewApiKey {
        account_id,
        key_hash,
        key_prefix: key_prefix.clone(),
        name: _req.name,
        is_active: true,
        rate_limit_per_minute: _req.rate_limit_per_minute.unwrap_or(60),
    };

    let api_key = repositories::create_api_key(&new_key, conn)?;

    Ok(GenerateApiKeyResponse {
        key: raw_key,
        key_prefix,
        key_id: api_key.id,
    })
}