use crate::user::model::{AuthBody, CreateUser, ResponseUser, UpdateUser};
use crate::user::repository::UserRepository;
use crate::util::error::LibError;
use uuid::Uuid;

#[derive(Debug)]
pub struct UserUseCase(Box<dyn UserRepository>);

impl UserUseCase {
    pub fn new(user_repository: Box<dyn UserRepository>) -> Self {
        Self(user_repository)
    }
    pub async fn create_user(&self, user: &CreateUser) -> Result<ResponseUser, LibError> {
        self.0.create_user(&user).await
    }
    pub async fn update_user(
        &self,
        id: &Uuid,
        user: &UpdateUser,
    ) -> Result<ResponseUser, LibError> {
        self.0.update_user(&id, &user).await
    }
    pub async fn delete_user(&self, id: &Uuid) -> Result<bool, LibError> {
        match self.0.delete_user(&id).await {
            Ok(r) => match r {
                true => Ok(true),
                false => Err(LibError::SqlxError(sqlx::Error::RowNotFound)),
            },
            Err(e) => Err(e),
        }
    }
    pub async fn get_user_by_id(&self, id: &Uuid) -> Result<ResponseUser, LibError> {
        self.0.get_user_by_id(&id).await
    }
    pub async fn get_users(&self) -> Result<Vec<ResponseUser>, LibError> {
        self.0.get_users().await
    }
    pub async fn login(&self, email: &String, password: &String) -> Result<AuthBody, LibError> {
        self.0.login(&email, &password).await
    }
}
