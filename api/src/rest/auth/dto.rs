use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use validator::Validate;

static RE_PASS: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]{6,32}$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct RequestLoginDto {
    #[validate(email(message = "invalid"))]
    #[validate(required(message = "missing"))]
    pub email: Option<String>,
    #[validate(regex(path = *RE_PASS, message = "invalid"))]
    #[validate(required(message = "missing"))]
    pub password: Option<String>,
}

impl RequestLoginDto {
    pub fn to_login(&self) -> (String, String) {
        (
            self.email.as_ref().unwrap().trim().to_string(),
            self.password.as_ref().unwrap().trim().to_string(),
        )
    }
}
