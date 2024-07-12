use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Role {
    Admin,
    User,
}

impl Role {
    pub fn from_str(s: &str) -> Option<Role> {
        let s = &s.to_lowercase();
        let s = s[..1].to_uppercase() + &s[1..];
        match s.as_str() {
            "Admin" => Some(Role::Admin),
            "User" => Some(Role::User),
            _ => None,
        }
    }
    pub fn to_str(&self) -> String {
        match self {
            Role::Admin => String::from("Admin"),
            Role::User => String::from("User"),
        }
    }
}
