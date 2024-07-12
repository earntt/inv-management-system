use super::repository::RoleRepository;
use crate::role::model::{RequestRole, ResponseRole};
use crate::util::error::LibError;
use uuid::Uuid;

#[derive(Debug)]
pub struct RoleUseCase(Box<dyn RoleRepository>);

impl RoleUseCase {
    pub fn new(role_repository: Box<dyn RoleRepository>) -> Self {
        Self(role_repository)
    }
    pub async fn create_role(&self, role: &RequestRole) -> Result<ResponseRole, LibError> {
        self.0.create_role(&role).await
    }
    pub async fn get_roles(&self) -> Result<Vec<ResponseRole>, LibError> {
        self.0.get_roles().await
    }
    pub async fn update_role(
        &self,
        id: &Uuid,
        role: &RequestRole,
    ) -> Result<ResponseRole, LibError> {
        self.0.update_role(&id, &role).await
    }
    pub async fn delete_role(&self, id: &Uuid) -> Result<bool, LibError> {
        match self.0.delete_role(&id).await {
            Ok(r) => match r {
                true => Ok(true),
                false => Err(LibError::SqlxError(sqlx::Error::RowNotFound)),
            },
            Err(e) => Err(e),
        }
    }
    pub async fn find_role(&self, role: &String) -> Result<Uuid, LibError> {
        self.0.find_role(&role).await
    }
    pub async fn get_role_by_id(&self, id: &Uuid) -> Result<ResponseRole, LibError> {
        self.0.get_role_by_id(&id).await
    }
}
