mod api;
mod db;
mod domain;
mod grpc;
use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use dotenv::dotenv;
use grpc::proto::ProductPackage::product_service_client::ProductServiceClient;
use std::sync::Arc;
use std::{env, str};
use tokio_postgres::{Client, Error, NoTls};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Sales Application!");
    dotenv().ok();

    //DB Configuration
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (db_client, connection) = tokio_postgres::connect(&database_url, NoTls)
        .await
        .expect("Failed to connect to the database");
    //Connection manages network communication asynchronously.It implements an async Stream that drives the underlying I/O.
    //For the client to work, this stream must be continuously polled to handle incoming and outgoing messages between your application and the PostgreSQL server.
    //By spawning connection as a separate task, you allow it to run independently in the background.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });

    //Tonic Client Configuration
    let catalog_service_grpc_url =
        env::var("CATALOG_SERVICE_GRPC_URL").expect("CATALOG_SERVICE_GRPC_URL must be set");
    let catalog_grpc_client = ProductServiceClient::connect(catalog_service_grpc_url)
        .await
        .expect("Failed to connect to gRPC server");

    //App State Configuration
    let shared_state = Arc::new(AppState {
        db_client,
        catalog_grpc_client,
    });

    //Axum Server Configuration
    let http_app = Router::new()
        .route("/orders/:customer_id", get(api::order_handlers::get_orders))
        .route("/products", get(api::order_handlers::get_products))
        .with_state(shared_state.clone());

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:5000").await?,
        http_app,
    )
    .await?;

    Ok(())
}

pub struct AppState {
    pub db_client: Client,
    pub catalog_grpc_client: ProductServiceClient<Channel>,
}
