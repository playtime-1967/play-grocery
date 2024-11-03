mod api;
mod db;
mod domain;
mod grpc;
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
    println!("Catalog Application!");

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_arc = Arc::new(PgPool::connect(&database_url).await?);

    //Axum Server Configuration
    let http_app = Router::new()
        .route(
            "/products",
            get(api::product_handlers::get_products).post(api::product_handlers::create_product),
        )
        .route(
            "/products2",
            get(api::product_handlers::get_products_with_category),
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
