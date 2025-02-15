mod api;
pub mod db;
mod domain;
mod grpc;
use axum::{
    routing::{get, post},
    Router,
};
use grpc::proto::ProductPackage::product_service_client::ProductServiceClient;
use std::env;
use std::sync::Arc;
use tokio_postgres::{Client, NoTls};
use tonic::transport::Channel;

pub struct AppState {
    pub db_client: Client,
    pub catalog_grpc_client: ProductServiceClient<Channel>,
}

pub async fn configure_database() -> Client {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let (db_client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");
    //connection manages network communication asynchronously and run independently in the background.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
    db_client
}

pub async fn configure_grpc_client() -> ProductServiceClient<Channel> {
    let catalog_service_grpc_url =
        env::var("CATALOG_SERVICE_GRPC_URL").expect("CATALOG_SERVICE_GRPC_URL must be set");
    ProductServiceClient::connect(catalog_service_grpc_url)
        .await
        .expect("Failed to connect to gRPC server")
}

pub fn create_router(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/orders/:customer_id",
            get(crate::api::order_handlers::get_orders),
        )
        .route("/products", get(crate::api::order_handlers::get_products))
        .route("/orders", post(crate::api::order_handlers::create_order))
        .with_state(shared_state)
}
