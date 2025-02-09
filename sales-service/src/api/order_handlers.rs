use crate::api::error_handler::AppError;
use crate::db::order_repo;
use crate::domain::models::{CreateOrderModel, OrderModel, ProductCategoryModel};
use crate::grpc::product_service;
use crate::AppState;
use anyhow::Result;
use axum::{extract::Path, extract::Query, extract::State, response::Json};
use serde::Deserialize;
use std::sync::Arc;

pub async fn create_order(
    State(app_state): State<Arc<AppState>>,
    Json(payload): Json<CreateOrderModel>,
) -> Result<(), AppError> {
    order_repo::create_order(&app_state.db_client, &payload).await?;

    Ok(())
}

pub async fn get_orders(
    State(app_state): State<Arc<AppState>>,
    Path(customer_id): Path<i64>,
) -> Result<Json<Vec<OrderModel>>, AppError> {
    let orders = order_repo::get_orders(&app_state.db_client, customer_id).await?;

    Ok(Json(orders))
}

pub async fn get_products(
    State(app_state): State<Arc<AppState>>,
    Query(query_params): Query<ProductQueryParams>,
) -> Result<Json<Vec<ProductCategoryModel>>, AppError> {
    let product_ids: Vec<i32> = query_params
        .product_ids
        .split(',')
        .filter_map(|id| id.parse::<i32>().ok())
        .collect();

    let mut catalog_grpc_client = app_state.catalog_grpc_client.clone();
    let products = product_service::get_products(&mut catalog_grpc_client, product_ids).await?;

    Ok(Json(products))
}

#[derive(Deserialize, Debug)]
pub struct ProductQueryParams {
    pub product_ids: String, // Receive as a String
}
