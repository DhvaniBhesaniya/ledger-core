use crate::{models::*, repositories, utils::app_error::AppError};
use diesel::PgConnection;

use crate::services::api_key_service;

pub fn create_account(
    req: CreateAccountRequest,
    conn: &mut PgConnection,
) -> Result<AccountCreationResponse, AppError> {
    let new_account = NewAccount {
        business_name: req.business_name,
        balance: 0,
        currency: req.currency.unwrap_or_else(|| "USD".to_string()),
        is_active: true,
    };

    let account = repositories::create_account(&new_account, conn)?;

    // Auto-generate Root API Key
    let key_req = GenerateApiKeyRequest {
        account_id: account.id,
        name: Some(format!("Api_Key_{}_{}", account.id, account.business_name)),
        rate_limit_per_minute: Some(60),
        role: Some("customer".to_string()),
    };

    let key_res = api_key_service::generate_key(key_req, conn)?;

    Ok(AccountCreationResponse {
        account: account.into(),
        secret_api_key: key_res.key,
    })
}

pub fn get_account(id: i64, conn: &mut PgConnection) -> Result<AccountResponse, AppError> {
    let account = repositories::get_account_by_id(id, conn)?;
    Ok(account.into())
}

pub fn get_balance(id: i64, conn: &mut PgConnection) -> Result<i64, AppError> {
    let account = repositories::get_account_by_id(id, conn)?;
    Ok(account.balance)
}
