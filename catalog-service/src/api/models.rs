use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)] //TODO object mapper
pub struct ProductModel {
    pub category_id: i16,
    pub name: String,
    pub price: f32,
}
