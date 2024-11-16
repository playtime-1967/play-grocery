use crate::domain::entities::{Category, Product};
use crate::domain::models::ProductCategoryModel;
use anyhow::{Ok, Result};
use sqlx::{PgPool, Row};

pub async fn get_products(pool: &PgPool) -> Result<Vec<Product>> {
    let customers = sqlx::query_as::<_, Product>("SELECT * FROM catalog.products")
        .fetch_all(pool)
        .await?;

    Ok(customers)
}

pub async fn get_products_with_category(
    pool: &PgPool,
    product_ids: Vec<i32>,
) -> Result<Vec<ProductCategoryModel>> {
    let orders = sqlx::query_as::<_, ProductCategoryModel>(
        "SELECT p.id , p.name, p.price, c.id as category_id, c.name as category_name
              FROM  catalog.products p INNER JOIN catalog.categories c ON p.category_id = c.id WHERE p.id = ANY($1)",
    )
    .bind(product_ids)
    .fetch_all(pool)
    .await?;

    Ok(orders)
}

pub async fn create_product(pool: &PgPool, product: &mut Product) -> Result<()> {
    let record = sqlx::query("INSERT INTO catalog.products ( category_id, name, price ) VALUES ( $1, $2, $3 ) RETURNING id")
    .bind(product.category_id)
    .bind(&product.name)
    .bind(product.price)
    .fetch_one(pool) 
    .await?;

    product.set_id(record.try_get("id")?);

    Ok(())
}
