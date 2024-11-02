use crate::api::error_handler::AppError;
use crate::db::order_repo;
use crate::domain::models::OrderModel;
use anyhow::Result;
use axum::{extract::State, extract::Path, response::Json};
use std::sync::Arc;
use tokio_postgres::Client;

pub async fn get_orders(
    State(db): State<Arc<Client>>,
    Path(customer_id): Path<i64>,
) -> Result<Json<Vec<OrderModel>>, AppError> {
    let orders = order_repo::get_orders(&db, customer_id).await?;
    Ok(Json(orders))
}
