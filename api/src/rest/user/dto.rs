use lib::role::entity::Role;
use lib::user::model::{CreateUser, UpdateUser};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

static RE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_.]{3,32}$").unwrap());
static RE_PASS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]{6,32}$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct RequestCreateUserDto {
    #[validate(regex(path = *RE_NAME , message = "invalid"))]
    #[validate(required(message = "missing"))]
    pub name: Option<String>,
    #[validate(email(message = "invalid"))]
    #[validate(required(message = "missing"))]
    pub email: Option<String>,
    #[validate(regex(path = *RE_PASS, message = "invalid"))]
    #[validate(required(message = "missing"))]
    pub password: Option<String>,
    #[validate(length(min = 1, message = "invalid"))]
    pub address: Option<String>,
    #[validate(length(min = 1, message = "invalid"))]
    pub role: Option<String>,
}

impl RequestCreateUserDto {
    pub fn to_create_user(&self) -> CreateUser {
        CreateUser {
            name: self.name.as_ref().unwrap().trim().to_string(),
            email: self.email.as_ref().unwrap().trim().to_string(),
            password: self.password.as_ref().unwrap().to_owned(),
            address: self
                .address
                .as_ref()
                .map_or("".to_string(), |address| address.trim().to_string()),
            role: self.role.as_ref().map_or(Role::User, |role| {
                Role::from_str(&role.trim().to_string()).unwrap_or(Role::User)
            }),
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct RequestUpdateUserDto {
    #[validate(regex(path = *RE_NAME, message = "invalid"))]
    pub name: Option<String>,
    #[validate(email(message = "invalid"))]
    pub email: Option<String>,
    #[validate(regex(path = *RE_PASS, message = "invalid"))]
    pub password: Option<String>,
    #[validate(length(min = 1, message = "invalid"))]
    pub address: Option<String>,
    #[validate(length(min = 1, message = "invalid"))]
    pub role: Option<String>,
}

impl RequestUpdateUserDto {
    pub fn to_update_user(&self) -> UpdateUser {
        UpdateUser {
            name: self.name.as_ref().map(|name| name.trim().to_string()),
            email: self.email.as_ref().map(|email| email.trim().to_string()),
            password: self.password.as_ref().map(|pwd| pwd.to_owned()),
            address: self
                .address
                .as_ref()
                .map(|address| address.trim().to_string()),
            role: self
                .role
                .as_ref()
                .map(|role| Role::from_str(&role.trim().to_string()))
                .flatten(),
        }
    }
}
