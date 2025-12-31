use crate::{models::*, repositories, utils::app_error::AppError, utils::crypto};
use diesel::PgConnection;
use uuid::Uuid;

pub fn generate_key(
    // _account_id: i64,
    req: GenerateApiKeyRequest,
    conn: &mut PgConnection,
) -> Result<GenerateApiKeyResponse, AppError> {
    // Generate random API key
    let prefix = std::env::var("API_KEY_PREFIX").unwrap();
    // let prefix = std::env::var("API_KEY_PREFIX").unwrap_or_else(|_| "sk_prod_".to_string());
    let raw_key = format!("{}{}", prefix, Uuid::new_v4().to_string());
    let key_hash = crypto::hash_api_key(&raw_key);
    let key_prefix = raw_key[..20].to_string();

    let new_key = NewApiKey {
        account_id: req.account_id,
        key_hash,
        key_prefix: key_prefix.to_owned(),
        name: req.name,
        is_active: true,
        rate_limit_per_minute: req.rate_limit_per_minute.unwrap_or(60),
    };

    let api_key = repositories::create_api_key(&new_key, conn)?;

    Ok(GenerateApiKeyResponse {
        key: raw_key,
        key_prefix,
        key_id: api_key.id,
    })
}

pub fn list_api_keys(
    account_id: i64,
    conn: &mut PgConnection,
) -> Result<Vec<ApiKeyResponse>, AppError> {
    let keys = repositories::list_api_keys_by_account(account_id, conn)?;
    Ok(keys.into_iter().map(Into::into).collect())
}

pub fn list_all_api_keys(conn: &mut PgConnection) -> Result<Vec<ApiKeyResponse>, AppError> {
    let keys = repositories::list_all_api_keys(conn)?;
    Ok(keys.into_iter().map(Into::into).collect())
}

pub fn update_api_key(
    key_id: i64,
    req: UpdateApiKeyRequest,
    conn: &mut PgConnection,
) -> Result<ApiKeyResponse, AppError> {
    let updated_key = repositories::update_api_key(
        key_id,
        req.name,
        req.rate_limit_per_minute,
        req.is_active,
        conn,
    )?;
    Ok(updated_key.into())
}
