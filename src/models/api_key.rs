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
    pub account_id: i64,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: Option<String>,
    pub is_active: Option<bool>,
    pub rate_limit_per_minute: i32,
    pub last_used_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = api_keys)]
pub struct NewApiKey {
    pub account_id: i64,
    pub key_hash: String,
    pub key_prefix: String,
    pub name: Option<String>,
    pub is_active: bool,
    pub rate_limit_per_minute: i32,
}

#[derive(Debug, Serialize)]
pub struct GenerateApiKeyResponse {
    pub key: String, // Only shown once
    pub key_prefix: String,
    pub key_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct GenerateApiKeyRequest {
    pub name: Option<String>,
    pub rate_limit_per_minute: Option<i32>,
}
