use sqlx::PgPool;
use tonic::{Request, Response, Status};
use ProductGrpcPackage::product_service_server::ProductService;
use ProductGrpcPackage::{GetProductsRequest, GetProductsResponse, Product};
pub mod ProductGrpcPackage {
    tonic::include_proto!("product");
}
use super::error_helper;
use crate::db::product_repo;
use std::sync::Arc;

#[derive(Debug)]
pub struct ProductGRPC {
    db: Arc<PgPool>,
}

impl ProductGRPC {
    pub fn new(db: Arc<PgPool>) -> Self {
        ProductGRPC { db }
    }
}

#[tonic::async_trait]
impl ProductService for ProductGRPC {
    async fn get_products(
        &self,
        request: Request<GetProductsRequest>,
    ) -> Result<Response<GetProductsResponse>, Status> {
        println!("Got a request: {:?}", request);
        let result = product_repo::get_products(&self.db).await;

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
