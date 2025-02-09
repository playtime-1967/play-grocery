mod api;
mod db;
mod domain;
mod grpc;
use crate::grpc::proto::ProductPackage;
use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::{env, str};
use tonic::transport::server;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db_arc = Arc::new(create_db_pool(&database_url).await?);

    //Axum Server Configuration
    let http_app = Router::new()
        .route(
            "/products",
            get(api::product_handlers::get_products).post(api::product_handlers::create_product),
        )
        .route("/categories", post(api::product_handlers::create_category))
        .with_state(db_arc.clone());

    //Tonic Server Configuration
    let product_grpc = grpc::product_service::ProductServiceImpl::new(db_arc.clone());
    let grpc_server = server::Server::builder().add_service(
        ProductPackage::product_service_server::ProductServiceServer::new(product_grpc),
    );

    // Run both servers, Axum and Tonic concurrently
    tokio::try_join!(
        async move {
            let addr = "0.0.0.0:1967";
            println!("Catalog HTTP server listening on {}", addr);
            axum::serve(tokio::net::TcpListener::bind(addr).await?, http_app)
                .await
                .map_err(|e| e.into())
        },
        async move {
            let addr = "0.0.0.0:1968";
            println!("Catalog gRPC server listening on {}", addr);
            grpc_server.serve(addr.parse()?).await.map_err(|e| e.into())
        }
    )?;

    Ok(())
}

async fn create_db_pool(database_url: &str) -> Result<sqlx::PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}
