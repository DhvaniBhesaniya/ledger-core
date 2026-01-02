use crate::{models::*, repositories, utils::app_error::AppError};
use diesel::PgConnection;
use serde_json::json;
use uuid::Uuid;

pub fn register_webhook(
    account_id: i64,
    req: RegisterWebhookRequest,
    conn: &mut PgConnection,
) -> Result<WebhookEndpointResponse, AppError> {
    let secret = format!("whsec_{}", Uuid::new_v4().to_string());
    let events = json!(req.events);

    let new_endpoint = NewWebhookEndpoint {
        account_id,
        url: req.url,
        secret,
        events,
        is_active: true,
        retry_max_attempts: 5,
    };

    let endpoint = repositories::create_webhook_endpoint(&new_endpoint, conn)?;

    Ok(WebhookEndpointResponse {
        id: endpoint.id,
        url: endpoint.url,
        events: serde_json::from_value(endpoint.events).unwrap_or_default(),
        is_active: endpoint.is_active,
        created_at: endpoint.created_at,
    })
}

pub fn get_webhook(id: i64, conn: &mut PgConnection) -> Result<WebhookEndpointResponse, AppError> {
    let endpoint = repositories::get_webhook_endpoints_by_account(id, conn)?
        .into_iter()
        .next()
        .ok_or(AppError::WebhookNotFound)?;

    Ok(WebhookEndpointResponse {
        id: endpoint.id,
        url: endpoint.url,
        events: serde_json::from_value(endpoint.events).unwrap_or_default(),
        is_active: endpoint.is_active,
        created_at: endpoint.created_at,
    })
}

pub fn delete_webhook(_id: i64, _conn: &mut PgConnection) -> Result<(), AppError> {
    // In production, implement soft delete
    Ok(())
}
