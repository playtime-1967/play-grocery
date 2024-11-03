use super::proto::ProductPackage::{
    product_service_client::ProductServiceClient, GetProductsRequest,
};
use crate::domain::models::ProductCategoryModel;
use anyhow::{Ok, Result};
use tonic::transport::Channel;


pub async fn get_products(
    client: &mut ProductServiceClient<Channel>,
    product_ids: Vec<i32>,
) -> Result<Vec<ProductCategoryModel>> {
    let request = tonic::Request::new(GetProductsRequest {
        product_ids: product_ids,
    });

    let response = client.get_products(request).await?;

    let products = response
        .into_inner()
        .product
        .iter()
        .map(|p| ProductCategoryModel {
            id: p.id,
            name: p.name.clone(),
            price: p.price,
            category_id: p.category_id,
            category_name: p.category_name.clone(),
        })
        .collect();

    Ok(products)
}
