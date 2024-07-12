use crate::role::model::{RequestRole, ResponseRole};
use crate::role::repository::RoleRepository;
use crate::util::error::LibError;
use async_trait::async_trait;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct PgRoleRepository {
    db_connect: Arc<Pool<Postgres>>,
}

impl PgRoleRepository {
    pub async fn new(db_connect: Arc<Pool<Postgres>>) -> Self {
        Self { db_connect }
    }
}

#[async_trait]
impl RoleRepository for PgRoleRepository {
    async fn create_role(&self, role: &RequestRole) -> Result<ResponseRole, LibError> {
        let query = sqlx::query_as!(
            ResponseRole,
            r#"
            INSERT INTO roles (name)
            VALUES ($1)
            RETURNING id, name;
            "#,
            &role.name
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query)
    }

    async fn get_roles(&self) -> Result<Vec<ResponseRole>, LibError> {
        let query = sqlx::query_as!(
            ResponseRole,
            r#"
            SELECT id, name FROM roles;
            "#
        )
        .fetch_all(self.db_connect.clone().as_ref())
        .await?;
        Ok(query)
    }

    async fn update_role(&self, id: &Uuid, role: &RequestRole) -> Result<ResponseRole, LibError> {
        let query = sqlx::query_as!(
            ResponseRole,
            r#"
            UPDATE roles
            SET name = $1,
                updated_at = now()
            WHERE id = $2
            RETURNING id, name;
            "#,
            &role.name,
            &id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query)
    }

    async fn delete_role(&self, id: &Uuid) -> Result<bool, LibError> {
        let query = sqlx::query!(
            r#"
            DELETE FROM roles WHERE id = $1
            "#,
            &id
        )
        .execute(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.rows_affected() > 0)
    }

    async fn find_role(&self, name: &String) -> Result<Uuid, LibError> {
        let query_result = sqlx::query_as!(
            ResponseRole,
            r#"
            SELECT id, name FROM roles WHERE name = $1;
            "#,
            &name,
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query_result.id)
    }

    async fn get_role_by_id(&self, id: &Uuid) -> Result<ResponseRole, LibError> {
        let query = sqlx::query_as!(
            ResponseRole,
            r#"
            SELECT id, name FROM roles WHERE id = $1;
            "#,
            &id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query)
    }
}

impl PgRoleRepository {
    pub async fn find_role(
        db_connect: &Arc<Pool<Postgres>>,
        name: &String,
    ) -> Result<Uuid, sqlx::Error> {
        let query_result = sqlx::query_as!(
            ResponseRole,
            r#"
            SELECT id, name FROM roles WHERE name = $1;
            "#,
            &name,
        )
        .fetch_one(db_connect.clone().as_ref())
        .await?;
        Ok(query_result.id)
    }
}
