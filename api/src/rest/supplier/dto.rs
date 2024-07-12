use lib::supplier::model::{CreateSupplier, UpdateSupplier};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static RE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_.]{3,32}$").unwrap());

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestCreateSupplierDto {
    #[validate(regex(path = *RE_NAME , message = "Invalid"))]
    pub name: String,
    #[validate(email(message = "Invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Invalid"))]
    pub phone: String,
    #[validate(length(min = 1, message = "Invalid"))]
    pub address: String,
}

impl RequestCreateSupplierDto {
    pub fn to_create_supplier(&self) -> CreateSupplier {
        CreateSupplier {
            name: self.name.trim().to_string(),
            email: self.email.trim().to_string(),
            phone: self.phone.trim().to_string(),
            address: self.address.trim().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestUpdateSupplierDto {
    #[validate(regex(path = *RE_NAME , message = "Invalid"))]
    pub name: Option<String>,
    #[validate(email(message = "Invalid"))]
    pub email: Option<String>,
    #[validate(length(min = 1, message = "Invalid"))]
    pub phone: Option<String>,
    #[validate(length(min = 1, message = "Invalid"))]
    pub address: Option<String>,
}

impl RequestUpdateSupplierDto {
    pub fn to_update_supplier(&self) -> UpdateSupplier {
        UpdateSupplier {
            name: self.name.as_ref().map(|name| name.trim().to_string()),
            email: self.email.as_ref().map(|email| email.trim().to_string()),
            phone: self.phone.as_ref().map(|phone| phone.trim().to_string()),
            address: self
                .address
                .as_ref()
                .map(|address| address.trim().to_string()),
        }
    }
}
