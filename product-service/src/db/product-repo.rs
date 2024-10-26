// use anyhow::{Ok, Result};
// use  

// pub struct ProductRepo {
// }

// impl ProductRepo {

//     // pub fn get_users() -> Vec<User> { //TODO? best practice for the db_repo response
//     //     let mut conn = establish_connection();
//     //     dsl::users
//     //         .order_by(dsl::username.desc())
//     //         // .order_by(dsl::id.asc())
//     //         .load::<User>(&mut conn)
//     //         .expect("get_users db issue")
//     // }
    
   


//     pub async fn create_product(new_product: &Product) -> Result<User> {

//         let row: (i32,) = sqlx::query_as(
//             "INSERT INTO product.products (category_id, name, price) VALUES ($1, $2,$2) RETURNING id"
//         )
//         .bind(new_product)
//         .bind(email)
//         .fetch_one(&self.pool)
//         .await?;
        
//         Ok(row.0)
//     }
// }


