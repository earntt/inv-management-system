use lib::material::model::{CreateMaterialGroup, UpdateMaterialGroup};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static RE_NAME: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_. ]{3,32}$").unwrap());

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestCreateMaterialGroupDto {
    #[validate(regex(path = *RE_NAME , message = "Invalid"))]
    pub name: String,
    #[validate(regex(path = *RE_NAME , message = "Invalid"))]
    pub sub_group: String,
}

impl RequestCreateMaterialGroupDto {
    pub fn to_create_material_group(&self) -> CreateMaterialGroup {
        CreateMaterialGroup {
            name: self.name.trim().to_string(),
            sub_group_name: self.sub_group.trim().to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestUpdateMaterialGroupDto {
    #[validate(regex(path = *RE_NAME , message = "Invalid"))]
    pub name: Option<String>,
    #[validate(regex(path = *RE_NAME , message = "Invalid"))]
    pub sub_group: Option<String>,
}

impl RequestUpdateMaterialGroupDto {
    pub fn to_update_material_group_by_id(&self) -> UpdateMaterialGroup {
        UpdateMaterialGroup {
            name: self.name.as_ref().map(|name| name.trim().to_string()),
            sub_group_name: self
                .sub_group
                .as_ref()
                .map(|sub_group| sub_group.trim().to_string()),
        }
    }
}
