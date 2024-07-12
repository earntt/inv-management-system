use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct UserInfo {
    pub sub: Uuid,
    pub name: String,
    pub email: String,
    pub roles: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct AuthInfo {
    pub user_info: UserInfo,
    pub token: String,
}
