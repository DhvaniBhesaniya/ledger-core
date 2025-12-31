use crate::models::{ApiKey, NewApiKey};
use crate::schema::api_keys;
use crate::utils::app_error::AppError;
use chrono::Utc;
use diesel::prelude::*;

pub fn create_api_key(new_key: &NewApiKey, conn: &mut PgConnection) -> Result<ApiKey, AppError> {
    diesel::insert_into(api_keys::table)
        .values(new_key)
        .get_result(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn get_api_key_by_hash(hash: &str, conn: &mut PgConnection) -> Result<ApiKey, AppError> {
    api_keys::table
        .filter(api_keys::key_hash.eq(hash))
        .filter(api_keys::is_active.eq(true))
        .first(conn)
        .map_err(|_| AppError::InvalidApiKey)
}

pub fn update_last_used(id: i64, conn: &mut PgConnection) -> Result<(), AppError> {
    diesel::update(api_keys::table.find(id))
        .set(api_keys::last_used_at.eq(Utc::now().naive_utc()))
        .execute(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(())
}

pub fn list_api_keys_by_account(
    account_id: i64,
    conn: &mut PgConnection,
) -> Result<Vec<ApiKey>, AppError> {
    api_keys::table
        .filter(api_keys::account_id.eq(account_id))
        .load::<ApiKey>(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn list_all_api_keys(conn: &mut PgConnection) -> Result<Vec<ApiKey>, AppError> {
    api_keys::table
        .load::<ApiKey>(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn update_api_key(
    key_id: i64,
    update_name: Option<String>,
    update_rate_limit: Option<i32>,
    update_is_active: Option<bool>,
    conn: &mut PgConnection,
) -> Result<ApiKey, AppError> {
    // Apply updates for each field that is provided
    if let Some(n) = update_name {
        diesel::update(api_keys::table.find(key_id))
            .set(api_keys::name.eq(n))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }

    if let Some(rl) = update_rate_limit {
        diesel::update(api_keys::table.find(key_id))
            .set(api_keys::rate_limit_per_minute.eq(rl))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }

    if let Some(active) = update_is_active {
        diesel::update(api_keys::table.find(key_id))
            .set(api_keys::is_active.eq(active))
            .execute(conn)
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    }

    // Return the updated record
    api_keys::table
        .find(key_id)
        .first(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
