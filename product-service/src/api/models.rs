use serde::Serialize;

#[derive(Serialize)] //TODO object mapper
pub struct ProductModel {
    pub category_id: i16,
    pub name: String,
    pub price: f32,
}

