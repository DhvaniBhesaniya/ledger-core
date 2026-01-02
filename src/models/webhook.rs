#![allow(dead_code)]
use crate::models::WebhookStatus;
use crate::schema::{webhook_endpoints, webhook_events};
use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = webhook_endpoints)]
pub struct WebhookEndpoint {
    pub id: i64,
    pub account_id: i64,
    pub url: String,
    pub secret: String,
    pub events: serde_json::Value,
    pub is_active: bool,
    pub retry_max_attempts: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = webhook_endpoints)]
pub struct NewWebhookEndpoint {
    pub account_id: i64,
    pub url: String,
    pub secret: String,
    pub events: serde_json::Value,
    pub is_active: bool,
    pub retry_max_attempts: i32,
}

#[derive(Debug, Deserialize)]
pub struct RegisterWebhookRequest {
    pub url: String,
    pub events: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct WebhookEndpointResponse {
    pub id: i64,
    pub url: String,
    pub events: Vec<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Queryable, Selectable)]
#[diesel(table_name = webhook_events)]
pub struct WebhookEvent {
    pub id: i64,
    pub webhook_endpoint_id: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub status: WebhookStatus,
    pub attempt_count: i32,
    pub next_retry_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = webhook_events)]
pub struct NewWebhookEvent {
    pub webhook_endpoint_id: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub status: WebhookStatus,
    pub attempt_count: i32,
}
