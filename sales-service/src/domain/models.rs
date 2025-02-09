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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderModel {
    pub id: i64,
    pub customer_id: i64,
    pub status: Status,
    pub created_date: DateTime<Utc>,
    pub delivery_date: Option<NaiveDate>,
    pub details: Vec<OrderDetailModel>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDetailModel {
    pub id: i64,
    pub product_id: i32,
    pub quantity: i16,
}

#[derive(Debug, Serialize)]
pub struct ProductCategoryModel {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub category_id: i32,
    pub category_name: String,
}
