use super::error_helper;
use crate::db::product_repo;
use sqlx::PgPool;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use super::proto::ProductPackage::product_service_server::ProductService;
use super::proto::ProductPackage::{GetProductsRequest, GetProductsResponse, Product};

#[derive(Debug)]
pub struct ProductServiceImpl {
    db: Arc<PgPool>,
}

impl ProductServiceImpl {
    pub fn new(db: Arc<PgPool>) -> Self {
        ProductServiceImpl { db }
    }
}

#[tonic::async_trait]
impl ProductService for ProductServiceImpl {
    async fn get_products(
        &self,
        request: Request<GetProductsRequest>,
    ) -> Result<Response<GetProductsResponse>, Status> {
        
        println!("got a request: {:?}", request);
        let result =
            product_repo::get_products_with_category(&self.db, request.into_inner().product_ids)
                .await;

        result
            .map(|products| {
                let response: GetProductsResponse = products.into();
                Response::new(response)
            })
            .map_err(error_helper::handle)

        //Or without error helper:
        // let products = product_repo::get_products(&self.db).await
        //                             .unwrap_or_else(|err| panic!("blah blah:{:?}", err));

        // let response: GetProductsResponse = products.into();
        // Ok(Response::new(response))
    }
}
