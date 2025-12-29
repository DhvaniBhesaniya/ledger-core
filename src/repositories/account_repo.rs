use diesel::prelude::*;
use crate::models::{Account, NewAccount};
use crate::schema::accounts;
use crate::error::AppError;

pub fn create_account(
    new_account: &NewAccount,
    conn: &mut PgConnection,
) -> Result<Account, AppError> {
    diesel::insert_into(accounts::table)
        .values(new_account)
        .get_result(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))
}

pub fn get_account_by_id(
    id: i64,
    conn: &mut PgConnection,
) -> Result<Account, AppError> {
    accounts::table
        .find(id)
        .first(conn)
        .map_err(|_| AppError::AccountNotFound)
}

pub fn debit_account(
    id: i64,
    amount: i64,
    conn: &mut PgConnection,
) -> Result<(), AppError> {
    // Check balance first
    let account = get_account_by_id(id, conn)?;
    if account.balance < amount {
        return Err(AppError::InsufficientBalance);
    }

    diesel::update(accounts::table.find(id))
        .set(accounts::balance.eq(accounts::balance - amount))
        .execute(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}

pub fn credit_account(
    id: i64,
    amount: i64,
    conn: &mut PgConnection,
) -> Result<(), AppError> {
    diesel::update(accounts::table.find(id))
        .set(accounts::balance.eq(accounts::balance + amount))
        .execute(conn)
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}