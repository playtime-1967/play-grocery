use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, FromRow)]
pub struct Category {
    pub id: i16,
    pub name: String,
}

#[derive(Debug, FromRow,Serialize)]
pub struct Product {
    pub id: i32,
    pub category_id: i16,
    pub name: String,
    pub price: f32,
}



