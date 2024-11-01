use super::product_service::{product_service, ProductPackage::GetProductsResponse};
use crate::domain;

impl From<domain::entities::Product> for ProductPackage::Product {
    fn from(product: domain::entities::Product) -> Self {
        ProductPackage::Product {
            id: product.id,
            name: product.name,
            price: product.price,
            category_id: product.category_id.into(),
            category_name: String::from("category_name"),
        }
    }
}

impl From<Vec<domain::entities::Product>> for GetProductsResponse {
    fn from(products: Vec<domain::entities::Product>) -> Self {
        GetProductsResponse {
            product: products
                .into_iter()
                .map(ProductPackage::Product::from)
                .collect(),
        }
    }
}
