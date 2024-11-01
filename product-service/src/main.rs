mod db;
mod domain;
mod grpc;
mod http;
use crate::grpc::product_service::ProductPackage;
use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::PgPool;
use std::sync::Arc;
use std::{env, str};
use tonic::transport::server;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Product Application!");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_arc = Arc::new(PgPool::connect(&database_url).await?);

    //Axum Server Configuration
    let http_app = Router::new()
        .route(
            "/products",
            get(http::product_handlers::get_products).post(http::product_handlers::create_product),
        )
        .with_state(db_arc.clone());

    //Tonic Server Configuration
    let product_grpc = grpc::product_service::ProductServiceImpl::new(db_arc.clone());
    let grpc_server = server::Server::builder().add_service(
        ProductPackage::product_service_server::ProductServiceServer::new(product_grpc),
    );

    // Run both servers concurrently
    tokio::try_join!(
        // Run the Axum server
        async move {
            axum::serve(
                tokio::net::TcpListener::bind("0.0.0.0:3000").await?,
                http_app,
            )
            .await
            .map_err(|e| e.into())
        },
        // Run the Tonic gRPC server
        async move {
            grpc_server
                .serve("0.0.0.0:4000".parse()?)
                .await
                .map_err(|e| e.into())
        }
    )?;

    Ok(())
}
