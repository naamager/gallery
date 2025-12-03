use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use chrono::NaiveDate;

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct Order {
    pub id_order: String,
    pub id_customer: String,
    pub order_date: NaiveDate,
}