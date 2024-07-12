use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryMaterialGroup {
    pub id: Uuid,
    pub name: String,
    pub sub_group_name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl QueryMaterialGroup {
    pub fn to_response_material_group(&self) -> ResponseMaterialGroup {
        ResponseMaterialGroup {
            id: self.id,
            name: self.name.clone(),
            sub_group_name: self.sub_group_name.clone(),
            created_at: self.created_at.unwrap(),
            updated_at: self.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMaterialGroup {
    pub name: String,
    pub sub_group_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMaterialGroup {
    pub name: Option<String>,
    pub sub_group_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMaterialGroup {
    pub id: Uuid,
    pub name: String,
    pub sub_group_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
