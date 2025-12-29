use crate::schema::accounts;
use chrono::NaiveDateTime;
use diesel::{
    Selectable,
    prelude::{Insertable, Queryable},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize)]
#[diesel(table_name = accounts)]
pub struct Account {
    pub id: i64,
    pub business_name: String,
    pub balance: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub business_name: String,
    pub balance: i64,
    pub currency: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub business_name: String,
    pub currency: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: i64,
    pub business_name: String,
    pub balance: i64,
    pub currency: String,
    pub is_active: bool,
}

impl From<Account> for AccountResponse {
    fn from(account: Account) -> Self {
        AccountResponse {
            id: account.id,
            business_name: account.business_name,
            balance: account.balance,
            currency: account.currency,
            is_active: account.is_active,
        }
    }
}
