use crate::role::model::{RequestRole, ResponseRole};
use crate::util::error::LibError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait RoleRepository: Send + Sync + std::fmt::Debug {
    async fn create_role(&self, role: &RequestRole) -> Result<ResponseRole, LibError>;
    async fn get_roles(&self) -> Result<Vec<ResponseRole>, LibError>;
    async fn update_role(&self, id: &Uuid, role: &RequestRole) -> Result<ResponseRole, LibError>;
    async fn delete_role(&self, id: &Uuid) -> Result<bool, LibError>;
    async fn find_role(&self, role: &String) -> Result<Uuid, LibError>;
    async fn get_role_by_id(&self, id: &Uuid) -> Result<ResponseRole, LibError>;
}
