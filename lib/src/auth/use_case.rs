use crate::auth::auth::Auth;
use crate::auth::model::AuthInfo;
use crate::auth::repository::AuthRepository;
use crate::util::error::LibError;

#[derive(Debug, Clone)]
pub struct AuthUseCase(Box<Auth>);

impl AuthUseCase {
    pub fn new(auth_repository: Box<Auth>) -> Self {
        Self(auth_repository)
    }
}

impl AuthUseCase {
    pub async fn get_info(&self, token: &str) -> Result<AuthInfo, LibError> {
        self.0.get_info(token).await
    }
    pub async fn role_check(
        &self,
        required_roles: Vec<&str>,
        user_role: String,
    ) -> Result<bool, LibError> {
        self.0.role_check(required_roles, user_role).await
    }
}
