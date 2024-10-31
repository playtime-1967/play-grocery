use super::models::ProductModel;
use crate::http::error_handler::AppError;
use crate::db::product_repo;
use crate::domain::entities::{Category, Product};
use anyhow::Result;
use axum::{extract::State, response::Json};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_products(State(db): State<Arc<PgPool>>) -> Result<Json<Vec<Product>>, AppError> {
    let products = product_repo::get_products(&db).await?;
    Ok(Json(products))
}

pub async fn create_product(
    State(db): State<Arc<PgPool>>,
    Json(payload): Json<ProductModel>,
) -> Result<Json<Product>, AppError> //-> (StatusCode, Json<User>)
{
    let mut new_product: Product = Product::new(payload.category_id, &payload.name, payload.price);
    product_repo::create_product(&db, &mut new_product).await?;

    Ok(Json(new_product)) //(StatusCode::CREATED, Json(user))
}
