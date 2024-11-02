use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Category {
    pub id: i16,
    pub name: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Product {
    pub id: i32,
    pub category_id: i16,
    pub name: String,
    pub price: f32,
}

impl Product {
    pub fn new(category_id: i16, name: &str, price: f32) -> Self {
        Product {
            category_id: category_id,
            name: name.into(),
            price: price,
            id: 0,
        }
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id
    }
}
