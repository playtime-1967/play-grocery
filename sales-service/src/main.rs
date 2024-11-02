mod api;
mod db;
mod domain;
use anyhow::{Ok, Result};
use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use std::sync::Arc;
use std::{env, str};
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Sales Application!");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let (client, connection) = tokio_postgres::connect(&database_url, NoTls)
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

    let db_arc = Arc::new(client);

    //Axum Server Configuration
    let http_app = Router::new()
        .route("/orders/:customer_id", get(api::order_handlers::get_orders))
        .with_state(db_arc.clone());

    axum::serve(
        tokio::net::TcpListener::bind("0.0.0.0:5000").await?,
        http_app,
    )
    .await?;

    Ok(())
}
