use anyhow::{Ok, Result};
use dotenv::dotenv;
use sales_service::{configure_database, configure_grpc_client, create_router, AppState};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Sales Application!");
    dotenv().ok();

    let db_client = configure_database().await;
    let catalog_grpc_client = configure_grpc_client().await;

    // Create shared app state
    let shared_state = Arc::new(AppState {
        db_client,
        catalog_grpc_client,
    });

    //Axum Server Configuration
    let http_app = create_router(shared_state);
    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:5000").await?,
        http_app,
    )
    .await?;

    Ok(())
}
