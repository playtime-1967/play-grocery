use crate::db::product_repo;
use crate::api::error_handler::AppError;
use crate::domain::entities::{Category, Product};
use anyhow::Result;
use sqlx::PgPool;
use axum::{response::{Json}, extract::State};

pub async fn get_products(State(pool): State<PgPool>) ->  Result<Json<Vec<Product>>, AppError> {

    let products = product_repo::get_products(&pool).await?;
    Ok(Json(products))
}


