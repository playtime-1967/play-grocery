use crate::domain::entities::{Category, Product};
use anyhow::{Ok, Result};
use sqlx::{PgPool};

pub async fn get_products(pool: &PgPool) -> Result<Vec<Product>> {
    let customers = sqlx::query_as::<_, Product>("SELECT * FROM product.products")
        .fetch_all(pool)
        .await?;

    Ok(customers)
}

