use crate::domain::entities::Status;
use crate::domain::models::OrderModel;
use bytes::BytesMut;
use std::error::Error;
use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
use tokio_postgres::Row;

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
    fn from_sql(_: &Type, raw: &[u8]) -> core::result::Result<Self, Box<dyn Error + Sync + Send>> {
        let status_str = std::str::from_utf8(raw)?;
        match status_str {
            "Active" => Ok(Status::Active),
            "Delivered" => Ok(Status::Delivered),
            _ => Err("Unknown status".into()),
        }
    }
    fn accepts(ty: &Type) -> bool {
        ty.name() == "order_status" //ensure it matches the custom enum type in PostgreSQL.
    }
}

impl ToSql for Status {
    fn to_sql(
        &self,
        _ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        let status_str = match self {
            Status::Active => "Active",
            Status::Delivered => "Delivered",
        };
        out.extend_from_slice(status_str.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool {
        ty.name() == "order_status"
    }

    fn to_sql_checked(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.to_sql(ty, out)
    }
}
