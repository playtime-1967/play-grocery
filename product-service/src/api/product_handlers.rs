use super::models::ProductModel;
use crate::api::error_handler::AppError;
use crate::db::product_repo;
use crate::domain::entities::{Category, Product};
use anyhow::Result;
use axum::{extract::State, response::Json};
use sqlx::PgPool;

pub async fn get_products(State(pool): State<PgPool>) -> Result<Json<Vec<Product>>, AppError> {
    let products = product_repo::get_products(&pool).await?;
    Ok(Json(products))
}

pub async fn create_product(
    State(pool): State<PgPool>,
    Json(payload): Json<ProductModel>,
) -> Result<Json<Product>, AppError> //-> (StatusCode, Json<User>)
{
    let mut new_product: Product = Product::new(payload.category_id, &payload.name, payload.price);
    product_repo::create_product(&pool, &mut new_product).await?;

    Ok(Json(new_product)) //(StatusCode::CREATED, Json(user))
}
