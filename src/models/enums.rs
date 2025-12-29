use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[db_enum(existing_type_path = "crate::schema::sql_types::TransactionType")]
pub enum TransactionType {
    Credit,
    Debit,
    Transfer,
}

#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[db_enum(existing_type_path = "crate::schema::sql_types::TransactionStatus")]
pub enum TransactionStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}

#[derive(DbEnum, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[db_enum(existing_type_path = "crate::schema::sql_types::WebhookStatus")]
pub enum WebhookStatus {
    Pending,
    Delivered,
    Failed,
}
