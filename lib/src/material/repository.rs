use crate::material::model::{CreateMaterialGroup, ResponseMaterialGroup, UpdateMaterialGroup};
use crate::util::error::LibError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait MaterialRepository: Send + Sync + std::fmt::Debug {
    async fn create_material_group(
        &self,
        material_group: &CreateMaterialGroup,
    ) -> Result<ResponseMaterialGroup, LibError>;
    async fn get_all_material_groups(&self) -> Result<Vec<ResponseMaterialGroup>, LibError>;
    async fn get_material_group_by_id(&self, id: Uuid) -> Result<ResponseMaterialGroup, LibError>;
    async fn get_sub_group_by_group_name(
        &self,
        group_name: &str,
    ) -> Result<Vec<ResponseMaterialGroup>, LibError>;
    async fn delete_material_group_by_id(&self, id: &Uuid) -> Result<bool, LibError>;
    async fn update_material_group_by_id(
        &self,
        id: &Uuid,
        material_group: &UpdateMaterialGroup,
    ) -> Result<ResponseMaterialGroup, LibError>;
}
