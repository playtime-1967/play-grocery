use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct ProductCategoryModel {
    pub id: i32,
    pub name: String,
    pub price: f32,
    pub category_id: i16,
    pub category_name: String,
}
