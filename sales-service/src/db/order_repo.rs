use crate::domain::entities::Status;
use crate::domain::models::OrderModel;
use std::error::Error;
use tokio_postgres::types::{FromSql, Type};
use tokio_postgres::Client;
use tokio_postgres::Row;

pub async fn get_orders(db: &Client, customer_id: i64) -> anyhow::Result<Vec<OrderModel>> {
    let result = db
        .query(
            "SELECT o.id , o.customer_id, o.status, o.created_date,
        o.delivery_date, d.id AS detail_id, d.product_id, d.quantity FROM sales.orders o
        INNER JOIN sales.order_details d ON o.id = d.order_id WHERE o.customer_id = $1;",
            &[&customer_id],
        )
        .await?;

    let orders: Vec<OrderModel> = result
        .into_iter()
        .map(
            |row| OrderModel::from(row), //Or without from func: OrderModel {...}
        )
        .collect();

    Ok(orders)
}

impl From<Row> for OrderModel {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            customer_id: row.get("customer_id"),
            status: row.get("status"),
            created_date: row.get("created_date"),
            delivery_date: row.get("delivery_date"),
            detail_id: row.get("detail_id"),
            product_id: row.get("product_id"),
            quantity: row.get("quantity"),
        }
    }
}

impl<'a> FromSql<'a> for Status {
    fn from_sql(ty: &Type, raw: &[u8]) -> core::result::Result<Self, Box<dyn Error + Sync + Send>> {
        let status_str = std::str::from_utf8(raw)?;
        match status_str {
            "Active" => Ok(Status::Active),
            "Delivered" => Ok(Status::Delivered),
            _ => Err("Unknown status".into()),
        }
    }
    // fn accepts(ty: &Type) -> bool {
    //     *ty == Type::TEXT Or Type::VARCHAR, depending on your PostgreSQL schema
    // }
    fn accepts(ty: &Type) -> bool {
        ty.name() == "order_status" //custom type
    }
}
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; //anything we define in the outer module is available to this tests module
    #[test]
    //#[should_panic]
    //#[ignore]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
