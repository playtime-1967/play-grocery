**Grocery App with Microservices**  


![Build Status](https://github.com/playtime-1967/play-grocery/actions/workflows/build-deploy-catalog-service.yml/badge.svg)
![Build Status](https://github.com/playtime-1967/play-grocery/actions/workflows/build-deploy-sales-service.yml/badge.svg) 

This project implements a grocery backend with two microservices:  
Catalog Service: Manages product catalogs.  
Sales Service: Handles orders.  

Each service uses different tech stacks, including Axum, SQLx, tokio-postgres, Tonic gRPC, and more. The system is designed REST APIs, and gRPC for synchronous communication.  

---------------------------------------------------------------------------------------------------------------------------
🛠️ **Prerequisites**  

1️⃣ Install Protobuf Compiler  

Before running the services, install `protobuf-compiler` to support gRPC communication.  
To install it on Debian:  

```
sudo apt-get install protobuf-compiler -y
```  
For other OS versions, check official gRPC installation docs.  

2️⃣ Setup Database and Apply Migrations  

Create a database in PostgreSQL.  
Set the connection string in the `.env` file of each microservice.  

```
DATABASE_URL=postgres://postgres:postgres@localhost/play-grocery-db
``` 


Run the migration scripts in each microservice.  
📌 Catalog Service (SQLx ORM)  

The migration scripts are located at:  `catalog-service/src/db/migrations`  

Before running the migration, install the sqlx-cli tool:

```
cargo install sqlx-cli --no-default-features --features rustls,postgres
```  

Run the migration in the correct directory: `cd catalog-service/src/db`

```
cargo sqlx migrate run
```

📖 [SQLx Documentation](https://docs.rs/sqlx/latest/sqlx/)

📌 Sales Service (tokio-postgres)
The migration scripts are located at: `sales-service/src/db/migrations`  

Since tokio-postgres does not support a migration CLI tool, apply migrations manually:
Maintain incremental naming for migration files:
```mig_000001.sql  
mig_000002.sql  
mig_000003.sql  
```

Copy and run each script in your PostgreSQL editor.  
📖 [tokio-postgres Documentation](https://docs.rs/tokio-postgres/latest/tokio_postgres/)

** You can skip all the database migration steps and directly run the pre-generated scripts located in the /raw directory for both microservices.

---------------------------------------------------------------------------------------------------------------------------
▶️ **How to Run Services**    

Navigate to each microservice directory and run:  
cd catalog-service  

``` 
cargo run
```

📌 Catalog HTTP Server: `0.0.0.0:1967`  
📌 Catalog gRPC Server: `0.0.0.0:1968`


cd sales-service
``` 
cargo run
```
📌 Sales HTTP Server: `0.0.0.0:1969`

---------------------------------------------------------------------------------------------------------------------------

🧪 **API Testing**       

REST API & gRPC Requests  
Postman & cURL request files are available in the `raw/` directory.  
If using Postman for gRPC requests, import the `protobuf` definition file:  

`proto/product.proto`

Catalog Service
REST API Endpoints:
```
POST /categories - Create a category
POST /products - Create a product
GET /products - Fetch products with related categories
```

gRPC Service:
```
GetProductsWithCategories
```

Sales Service
REST API Endpoints:

```
POST /orders - Create an order
GET /orders/{customer_id} - Get orders for a customer
GET /products - Calls Catalog Service via gRPC
```
---------------------------------------------------------------------------------------------------------------------------

⚙️ **Environment Variables**   

Catalog Service:  

```
DATABASE_URL=postgres://postgres:postgres@localhost/play-grocery-db
```  

Sales Service:

```
DATABASE_URL=postgres://postgres:postgres@localhost/play-grocery-db
CATALOG_SERVICE_GRPC_URL=http://127.0.0.1:1968/
```
---------------------------------------------------------------------------------------------------------------------------
🐳 **Build Pipelines**  

Each microservice has its own build pipeline. After building the Docker images, they are automatically pushed to my [Docker Hub repository](https://hub.docker.com/u/namnik).
