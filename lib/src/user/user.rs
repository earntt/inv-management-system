use crate::role::role::PgRoleRepository;
use crate::user::model::{
    AuthBody, CreateUser, CurrentUser, Id, QueryUser, ResponseUser, UpdateUser,
};
use crate::user::repository::UserRepository;
use crate::util::error::LibError;
use async_trait::async_trait;
use config::Keys;
use jsonwebtoken::{encode, Algorithm, Header};
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct PgUserRepository {
    db_connect: Arc<Pool<Postgres>>,
}

impl PgUserRepository {
    pub async fn new(db_connect: Arc<Pool<Postgres>>) -> Self {
        Self { db_connect }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError> {
        let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
        Ok(hashed_password)
    }

    async fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, bcrypt::BcryptError> {
        let result = bcrypt::verify(password, hash)?;
        Ok(result)
    }

    async fn create_user(&self, user: &CreateUser) -> Result<ResponseUser, LibError> {
        let hashed_password = self
            .hash_password(&user.password)
            .await
            .map_err(|e| LibError::BcryptError(e))?;
        let role_id = PgRoleRepository::find_role(&self.db_connect, &user.role.to_str())
            .await
            .map_err(|e| LibError::SqlxError(e))?;
        let id = sqlx::query_as!(
            Id,
            r#"
            INSERT INTO users (name, email, hash, address, role_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id;
            "#,
            &user.name,
            &user.email,
            &hashed_password,
            &user.address,
            &role_id,
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        let query = self.get_user_by_id(&id.id).await?;
        Ok(query)
    }

    async fn update_user(&self, id: &Uuid, user: &UpdateUser) -> Result<ResponseUser, LibError> {
        let hashed_password = match &user.password {
            Some(password) => Some(
                self.hash_password(password)
                    .await
                    .map_err(|e| LibError::BcryptError(e))?,
            ),
            None => None,
        };
        let role_id = match &user.role {
            Some(role) => Some(
                PgRoleRepository::find_role(&self.db_connect, &role.to_str())
                    .await
                    .map_err(|e| LibError::SqlxError(e))?,
            ),
            None => None,
        };
        sqlx::query!(
            r#"
            UPDATE users
            SET name = COALESCE($1, name), 
                email = COALESCE($2, email), 
                hash = COALESCE($3, hash), 
                address = COALESCE($4, address),
                role_id = COALESCE($5, role_id),
                updated_at = now()
            WHERE id = $6;
            "#,
            user.name.as_ref(),
            user.email.as_ref(),
            hashed_password.as_ref(),
            user.address.as_ref(),
            role_id.as_ref(),
            &id
        )
        .execute(self.db_connect.clone().as_ref())
        .await?;
        let query = self.get_user_by_id(id).await?;
        Ok(query)
    }

    async fn delete_user(&self, id: &Uuid) -> Result<bool, LibError> {
        let query = sqlx::query!(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
            &id
        )
        .execute(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.rows_affected() > 0)
    }

    async fn get_user_by_id(&self, id: &Uuid) -> Result<ResponseUser, LibError> {
        let query = sqlx::query_as!(QueryUser,
            r#"
            SELECT u.id, u.name, u.email, u.address, u.role_id, r.name as role_name, u.created_at, u.updated_at FROM users as u
            INNER JOIN roles as r
            ON u.role_id = r.id
            WHERE u.id = $1;
            "#,
            &id
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query.to_response_user())
    }

    async fn get_users(&self) -> Result<Vec<ResponseUser>, LibError> {
        let query = sqlx::query_as!(
            QueryUser,
            r#"
            SELECT u.id, u.name, u.email, u.address, u.role_id, r.name as role_name, u.created_at, u.updated_at
            FROM users as u
            INNER JOIN roles as r
            ON u.role_id = r.id;
            "#
        )
        .fetch_all(self.db_connect.clone().as_ref())
        .await?;
        let result: Vec<ResponseUser> = query
            .into_iter()
            .map(|query: QueryUser| query.to_response_user())
            .collect();
        Ok(result)
    }

    async fn get_user_by_email(&self, email: &String) -> Result<CurrentUser, LibError> {
        let query = sqlx::query_as!(
            CurrentUser,
            r#"
            SELECT u.id, u.name, u.email, u.hash as hashed_password, r.name as role_name
            FROM users as u
            INNER JOIN roles as r
            ON u.role_id = r.id
            WHERE u.email = $1;
            "#,
            &email
        )
        .fetch_one(self.db_connect.clone().as_ref())
        .await?;
        Ok(query)
    }

    async fn login(&self, email: &String, password: &String) -> Result<AuthBody, LibError> {
        let user = self
            .get_user_by_email(email)
            .await
            .map_err(|_| LibError::Unauthorized("Invalid email or password".to_string()))?;

        let is_valid = self
            .verify_password(password, &user.hashed_password)
            .await
            .map_err(|e| LibError::BcryptError(e))?;

        if !is_valid {
            return Err(LibError::Unauthorized(
                "Invalid email or password".to_string(),
            ));
        }

        let token = encode(
            &Header::new(Algorithm::HS256),
            &user.to_claims(),
            &Keys::default().encoding,
        )
        .map_err(|e| LibError::JwtError(e))?;

        Ok(AuthBody { token })
    }
}
