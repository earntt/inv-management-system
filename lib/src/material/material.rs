use crate::material::model::{
    CreateMaterialGroup, QueryMaterialGroup, ResponseMaterialGroup, UpdateMaterialGroup,
};
use crate::material::repository::MaterialRepository;
use crate::util::error::LibError;
use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct PgMaterialRepository {
    db_connect: Arc<Pool<Postgres>>,
}

impl PgMaterialRepository {
    pub async fn new(db_connect: Arc<Pool<Postgres>>) -> Self {
        Self { db_connect }
    }
}

#[async_trait]
impl MaterialRepository for PgMaterialRepository {
    async fn create_material_group(
        &self,
        material_group: &CreateMaterialGroup,
    ) -> Result<ResponseMaterialGroup, LibError> {
        let query = sqlx::query_as!(
            QueryMaterialGroup,
            r#"
            INSERT INTO material_group (name, sub_group_name)
            VALUES ($1, $2)
            RETURNING *
            "#,
            material_group.name,
            material_group.sub_group_name
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_material_group())
    }

    async fn get_all_material_groups(&self) -> Result<Vec<ResponseMaterialGroup>, LibError> {
        let query = sqlx::query_as!(
            QueryMaterialGroup,
            r#"
            SELECT * FROM material_group
            "#
        )
        .fetch_all(self.db_connect.clone().as_ref())
        .await?;
        let result: Vec<ResponseMaterialGroup> = query
            .into_iter()
            .map(|query: QueryMaterialGroup| query.to_response_material_group())
            .collect();
        Ok(result)
    }

    async fn get_material_group_by_id(&self, id: Uuid) -> Result<ResponseMaterialGroup, LibError> {
        let query = sqlx::query_as!(
            QueryMaterialGroup,
            r#"
            SELECT * FROM material_group WHERE id = $1
            "#,
            id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_material_group())
    }

    async fn get_sub_group_by_group_name(
        &self,
        group_name: &str,
    ) -> Result<Vec<ResponseMaterialGroup>, LibError> {
        let query = sqlx::query_as!(
            QueryMaterialGroup,
            r#"
            SELECT * FROM material_group WHERE name = $1
            "#,
            group_name
        )
        .fetch_all(self.db_connect.clone().as_ref())
        .await?;
        let result: Vec<ResponseMaterialGroup> = query
            .into_iter()
            .map(|query: QueryMaterialGroup| query.to_response_material_group())
            .collect();
        Ok(result)
    }

    async fn delete_material_group_by_id(&self, id: &Uuid) -> Result<bool, LibError> {
        let query = sqlx::query!(
            r#"
            DELETE FROM material_group WHERE id = $1
            "#,
            &id
        )
        .execute(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.rows_affected() > 0)
    }

    async fn update_material_group_by_id(
        &self,
        id: &Uuid,
        material_group: &UpdateMaterialGroup,
    ) -> Result<ResponseMaterialGroup, LibError> {
        let query = sqlx::query_as!(
            QueryMaterialGroup,
            r#"
            UPDATE material_group
            SET name = COALESCE($1, name),
                sub_group_name = COALESCE($2, sub_group_name),
                updated_at = now()
            WHERE id = $3
            RETURNING *
            "#,
            material_group.name,
            material_group.sub_group_name,
            id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_material_group())
    }
}
