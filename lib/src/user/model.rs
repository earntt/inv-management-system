use crate::role::entity::Role;
use chrono::{Duration, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Id {
    pub id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub name: String,
    pub email: String,
    pub roles: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub hashed_password: String,
    pub role_name: String,
}

impl CurrentUser {
    pub fn to_claims(&self) -> Claims {
        let exp_duration = Duration::minutes(5);
        Claims {
            sub: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            roles: self.role_name.clone(),
            iat: (Utc::now()).timestamp() as usize,
            exp: (Utc::now() + exp_duration).timestamp() as usize,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub role_id: Uuid,
    pub role_name: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl QueryUser {
    pub fn to_response_user(&self) -> ResponseUser {
        ResponseUser {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            address: self.address.clone().unwrap_or("".to_string()),
            role_id: self.role_id,
            role_name: self.role_name.clone(),
            created_at: self.created_at.unwrap().to_string(),
            updated_at: self.updated_at.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
    pub address: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub address: Option<String>,
    pub role: Option<Role>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseUser {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub address: String,
    pub role_id: Uuid,
    pub role_name: String,
    pub created_at: String,
    pub updated_at: String,
}
