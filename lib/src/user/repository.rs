use crate::user::model::{AuthBody, CreateUser, CurrentUser, ResponseUser, UpdateUser};
use crate::util::error::LibError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync + std::fmt::Debug {
    async fn hash_password(&self, password: &str) -> Result<String, bcrypt::BcryptError>;
    async fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<bool, bcrypt::BcryptError>;
    async fn create_user(&self, user: &CreateUser) -> Result<ResponseUser, LibError>;
    async fn update_user(&self, id: &Uuid, user: &UpdateUser) -> Result<ResponseUser, LibError>;
    async fn delete_user(&self, id: &Uuid) -> Result<bool, LibError>;
    async fn get_user_by_id(&self, id: &Uuid) -> Result<ResponseUser, LibError>;
    async fn get_users(&self) -> Result<Vec<ResponseUser>, LibError>;
    async fn get_user_by_email(&self, email: &String) -> Result<CurrentUser, LibError>;
    async fn login(&self, email: &String, password: &String) -> Result<AuthBody, LibError>;
}
