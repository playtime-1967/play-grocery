use crate::domain::models::{CreateOrderModel, OrderModel};
use tokio_postgres::Client;

pub async fn create_order(db: &Client, order: &CreateOrderModel) -> anyhow::Result<()> {
    //TODO: support db transaction.
    db.execute(
        "INSERT INTO sales.orders (id, customer_id, status, created_date, delivery_date)
            VALUES ($1, $2, $3, $4, $5);",
        &[
            &order.id,
            &order.customer_id,
            &order.status,
            &order.created_date,
            &order.delivery_date,
        ],
    )
    .await?;

    for detail in &order.details {
        db.execute(
            "INSERT INTO sales.order_details (id, order_id, product_id, quantity) VALUES ($1, $2, $3, $4);",
            &[ &detail.id, &order.id, &detail.product_id, &detail.quantity],
        )
        .await?;
    }

    Ok(())
}

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
        .map(|row| OrderModel::from(row))
        .collect();

    Ok(orders)
}

//---------------------------------------------------------------------
//TODO: write unit tests.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
