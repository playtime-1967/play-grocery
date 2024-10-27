mod api;
mod db;
mod domain;
use anyhow::{Ok, Result};
use axum::{routing::get, Router};
use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {

    println!("Product API!");
    
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url).await?;

    let app = Router::new()
        .route("/products", get(api::product_handlers::get_products))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
