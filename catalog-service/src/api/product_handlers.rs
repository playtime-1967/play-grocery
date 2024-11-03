use crate::api::error_handler::AppError;
use crate::domain::models::ProductModel;
use crate::db::product_repo;
use crate::domain::entities::{Category, Product};
use crate::domain::models::{ProductCategoryModel};

use anyhow::Result;
use axum::{extract::State, response::Json};
use sqlx::PgPool;
use std::sync::Arc;

pub async fn get_products(State(db): State<Arc<PgPool>>) -> Result<Json<Vec<Product>>, AppError> {
    let products = product_repo::get_products(&db).await?;
    Ok(Json(products))
}

pub async fn get_products_with_category(State(db): State<Arc<PgPool>>) -> Result<Json<Vec<ProductCategoryModel>>, AppError> {
    let products = product_repo::get_products_with_category(&db,vec![2,3]).await?;
    Ok(Json(products))
}

pub async fn create_product(
    State(db): State<Arc<PgPool>>,
    Json(payload): Json<ProductModel>,
) -> Result<Json<Product>, AppError> //-> (StatusCode, Json<Product>)
{
    let mut new_product: Product = Product::new(payload.category_id, &payload.name, payload.price);
    product_repo::create_product(&db, &mut new_product).await?;

    Ok(Json(new_product)) //(StatusCode::CREATED, Json(new_product))
}
