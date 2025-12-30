use crate::{models::*, repositories, utils::app_error::AppError};
use diesel::PgConnection;

pub fn create_account(
    req: CreateAccountRequest,
    conn: &mut PgConnection,
) -> Result<AccountResponse, AppError> {
    let new_account = NewAccount {
        business_name: req.business_name,
        balance: 0,
        currency: req.currency.unwrap_or_else(|| "USD".to_string()),
        is_active: true,
    };

    let account = repositories::create_account(&new_account, conn)?;
    Ok(account.into())
}

pub fn get_account(id: i64, conn: &mut PgConnection) -> Result<AccountResponse, AppError> {
    let account = repositories::get_account_by_id(id, conn)?;
    Ok(account.into())
}

pub fn get_balance(id: i64, conn: &mut PgConnection) -> Result<i64, AppError> {
    let account = repositories::get_account_by_id(id, conn)?;
    Ok(account.balance)
}
