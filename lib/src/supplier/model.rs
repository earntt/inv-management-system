use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct QuerySupplier {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl QuerySupplier {
    pub fn to_response_supplier(&self) -> ResponseSupplier {
        ResponseSupplier {
            id: self.id,
            name: self.name.clone(),
            email: self.email.clone(),
            phone: self.phone.clone(),
            address: self.address.clone(),
            created_at: self.created_at.unwrap().to_string(),
            updated_at: self.updated_at.unwrap().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSupplier {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateSupplier {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseSupplier {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
    pub created_at: String,
    pub updated_at: String,
}
