#![allow(dead_code)]
use crate::models::{
    NewWebhookEndpoint, NewWebhookEvent, WebhookEndpoint, WebhookEvent, WebhookStatus,
};
use crate::schema::{webhook_endpoints, webhook_events};
use crate::utils::app_error::AppError;
use diesel::prelude::*;

pub fn create_webhook_endpoint(
    new_endpoint: &NewWebhookEndpoint,
    conn: &mut PgConnection,
) -> Result<WebhookEndpoint, AppError> {
    diesel::insert_into(webhook_endpoints::table)
        .values(new_endpoint)
        .get_result(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn get_webhook_endpoints_by_account(
    account_id: i64,
    conn: &mut PgConnection,
) -> Result<Vec<WebhookEndpoint>, AppError> {
    webhook_endpoints::table
        .filter(webhook_endpoints::account_id.eq(account_id))
        .load(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn create_webhook_event(
    new_event: &NewWebhookEvent,
    conn: &mut PgConnection,
) -> Result<WebhookEvent, AppError> {
    diesel::insert_into(webhook_events::table)
        .values(new_event)
        .get_result(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn get_pending_webhook_events(conn: &mut PgConnection) -> Result<Vec<WebhookEvent>, AppError> {
    webhook_events::table
        .filter(webhook_events::status.eq(WebhookStatus::Pending))
        .load(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}
