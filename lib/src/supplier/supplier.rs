use crate::supplier::model::{CreateSupplier, ResponseSupplier, UpdateSupplier};
use crate::supplier::repository::SupplierRepository;
use crate::util::error::LibError;
use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

use super::model::QuerySupplier;

#[derive(Debug)]
pub struct PgSupplierRepository {
    // db_connection
    db_connect: Arc<Pool<Postgres>>,
}

impl PgSupplierRepository {
    pub async fn new(db_connect: Arc<Pool<Postgres>>) -> Self {
        Self { db_connect }
    }
}

#[async_trait]
impl SupplierRepository for PgSupplierRepository {
    async fn get_all_suppliers(&self) -> Result<Vec<ResponseSupplier>, LibError> {
        let query = sqlx::query_as!(
            QuerySupplier,
            r#"
            SELECT * FROM supplier
            "#
        )
        .fetch_all(self.db_connect.clone().as_ref())
        .await?;
        let result: Vec<ResponseSupplier> = query
            .into_iter()
            .map(|query: QuerySupplier| query.to_response_supplier())
            .collect();
        Ok(result)
    }

    async fn create_supplier(&self, user: &CreateSupplier) -> Result<ResponseSupplier, LibError> {
        let query = sqlx::query_as!(
            QuerySupplier,
            r#"
            INSERT INTO supplier (name, address, phone, email)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            user.name,
            user.address,
            user.phone,
            user.email
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_supplier())
    }

    async fn get_supplier_by_id(&self, id: &uuid::Uuid) -> Result<ResponseSupplier, LibError> {
        let query = sqlx::query_as!(
            QuerySupplier,
            r#"
            SELECT * FROM supplier
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_supplier())
    }

    async fn delete_supplier_by_id(&self, id: &Uuid) -> Result<bool, LibError> {
        let query = sqlx::query_as!(
            QuerySupplier,
            r#"
            DELETE FROM supplier 
            WHERE id = $1
            "#,
            &id
        )
        .execute(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.rows_affected() > 0)
    }

    async fn update_supplier_by_id(
        &self,
        id: &Uuid,
        user: &UpdateSupplier,
    ) -> Result<ResponseSupplier, LibError> {
        let query = sqlx::query_as!(
            QuerySupplier,
            r#"
            UPDATE supplier
            SET name = COALESCE($1, name),
                address = COALESCE($2, address), 
                phone = COALESCE($3, phone),
                email = COALESCE($4, email),
                updated_at = now()
            WHERE id = $5
            RETURNING *
            "#,
            user.name,
            user.address,
            user.phone,
            user.email,
            id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_supplier())
    }
}
