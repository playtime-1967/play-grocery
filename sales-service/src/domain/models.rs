use crate::domain::entities::Status;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct OrderModel {
    pub id: i64,
    pub customer_id: i64,
    pub status: Status,
    pub created_date: DateTime<Utc>,
    pub delivery_date: Option<NaiveDate>,
    pub detail_id: i64,
    pub product_id: i32,
    pub quantity: i16,
}
