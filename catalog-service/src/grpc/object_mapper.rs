use super::product_service::{ProductPackage, ProductPackage::GetProductsResponse};
use crate::domain;

impl From<domain::models::ProductCategoryModel> for ProductPackage::Product {
    fn from(product: domain::models::ProductCategoryModel) -> Self {
        ProductPackage::Product {
            id: product.id,
            name: product.name,
            price: product.price,
            category_id: product.category_id.into(),
            category_name: product.category_name
        }
    }
}

impl From<Vec<domain::models::ProductCategoryModel>> for GetProductsResponse {
    fn from(products: Vec<domain::models::ProductCategoryModel>) -> Self {
        GetProductsResponse {
            product: products
                .into_iter()
                .map(ProductPackage::Product::from)
                .collect(),
        }
    }
}
