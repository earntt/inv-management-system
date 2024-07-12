use super::model::AuthInfo;
use crate::util::error::LibError;
use async_trait::async_trait;

#[async_trait]
pub trait AuthRepository: Send + Sync + std::fmt::Debug + Clone {
    async fn get_info(&self, token: &str) -> Result<AuthInfo, LibError>;
    async fn role_check(
        &self,
        required_roles: Vec<&str>,
        user_role: String,
    ) -> Result<bool, LibError>;
}
