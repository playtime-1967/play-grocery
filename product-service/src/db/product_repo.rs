use crate::domain::entities::{Category, Product};
use anyhow::{Ok, Result};
use sqlx::PgPool;

pub async fn get_products(pool: &PgPool) -> Result<Vec<Product>> {
    let customers = sqlx::query_as::<_, Product>("SELECT * FROM product.products")
        .fetch_all(pool)
        .await?;

    Ok(customers)
}

pub async fn create_product(pool: &PgPool, product: &mut Product) -> Result<()> {
    let record = sqlx::query!(
        r#"INSERT INTO product.products ( category_id, name, price ) VALUES ( $1, $2, $3 ) RETURNING id"#,
        product.category_id,
        product.name,
        product.price
    )
    .fetch_one(pool)
    .await?;

    product.set_id(record.id);

    Ok(())
}
