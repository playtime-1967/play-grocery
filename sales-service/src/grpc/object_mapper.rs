use super::proto::{ProductPackage};
use crate::domain;

impl From<ProductPackage::Product> for domain::models::ProductCategoryModel {
    fn from(product: ProductPackage::Product) -> Self {
        domain::models::ProductCategoryModel {
            id: product.id,
            name: product.name,
            price: product.price,
            category_id: product.category_id,
            category_name: product.category_name
        }
    }
}
