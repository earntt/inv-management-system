use lib::role::model::RequestRole;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static RE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_.]{3,32}$").unwrap());

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestRoleDto {
    #[validate(regex(path = *RE_NAME, message = "invalid"))]
    pub name: String,
}

impl RequestRoleDto {
    pub fn to_request_role(&self) -> RequestRole {
        RequestRole {
            name: self.name.trim().to_string(),
        }
    }
}
