#![allow(dead_code)]
use crate::schema::api_keys;
use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = api_keys)]
pub struct ApiKey {
    pub id: i64,
    pub account_id: Option<i64>, // Nullable for admin keys
    pub key_hash: String,
    pub key_prefix: String,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub rate_limit_per_minute: i32,
    pub last_used_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey {
    pub account_id: Option<i64>, // Nullable for admin keys
    pub key_hash: String,
    pub key_prefix: String,
    pub name: Option<String>,
    pub is_active: bool,
    pub rate_limit_per_minute: i32,
    pub role: String,
}

#[derive(Debug, Serialize)]
pub struct GenerateApiKeyResponse {
    pub key: String, // Only shown once
    pub key_prefix: String,
    pub key_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GenerateApiKeyRequest {
    pub account_id: i64,
    pub name: Option<String>,
    pub rate_limit_per_minute: Option<i32>,
    pub role: Option<String>, // "admin" or "customer", defaults to "customer"
}

#[derive(Debug, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub rate_limit_per_minute: Option<i32>,
    pub is_active: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct ApiKeyResponse {
    pub id: i64,
    pub account_id: Option<i64>, // Nullable for admin keys
    pub key_prefix: String,
    pub name: Option<String>,
    pub is_active: bool,
    pub rate_limit_per_minute: i32,
    pub last_used_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub role: String,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(key: ApiKey) -> Self {
        ApiKeyResponse {
            id: key.id,
            account_id: key.account_id,
            key_prefix: key.key_prefix,
            name: key.name,
            is_active: key.is_active.unwrap_or(true),
            rate_limit_per_minute: key.rate_limit_per_minute,
            last_used_at: key.last_used_at,
            created_at: key.created_at,
            role: key.role,
        }
    }
}
