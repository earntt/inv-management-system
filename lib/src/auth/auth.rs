use async_trait::async_trait;
use jsonwebtoken::{decode, Validation};

use crate::util::error::LibError;

use super::model::{AuthInfo, UserInfo};
use super::repository::AuthRepository;
use config::Keys;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Auth {}

impl Auth {
    pub async fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl AuthRepository for Auth {
    async fn get_info(&self, token: &str) -> Result<AuthInfo, LibError> {
        let token = token.to_string();
        let user_info: UserInfo = decode(&token, &Keys::default().decoding, &Validation::default())
            .map_err(|e| LibError::JwtError(e))?
            .claims;

        Ok(AuthInfo { user_info, token })
    }
    async fn role_check(
        &self,
        required_roles: Vec<&str>,
        user_role: String,
    ) -> Result<bool, LibError> {
        Ok(required_roles.contains(&user_role.as_str()))
    }
}
