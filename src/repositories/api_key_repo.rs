use crate::error::AppError;
use crate::models::{ApiKey, NewApiKey};
use crate::schema::api_keys;
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
