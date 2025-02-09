use serde::{Serialize,Deserialize};
use tokio_postgres::types;

pub struct Order {
    pub id: i64,
    pub customer_id: i64,
    pub status: Status,
    pub created_date: types::Timestamp<String>,
    pub delivery_date: types::Date<String>,
}

pub struct OrderDetail {
    pub id: i64,
    pub order_id: i64,
    pub product_id: i32,
    pub quantity: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Status {
    Active,
    Delivered,
}
