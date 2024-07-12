use crate::material::model::{CreateMaterialGroup, ResponseMaterialGroup, UpdateMaterialGroup};
use crate::material::repository::MaterialRepository;
use crate::util::error::LibError;
use uuid::Uuid;

#[derive(Debug)]
pub struct MaterialUseCase(Box<dyn MaterialRepository>);

impl MaterialUseCase {
    pub fn new(repository: Box<dyn MaterialRepository>) -> Self {
        MaterialUseCase(repository)
    }

    pub async fn create_material_group(
        &self,
        material_group: &CreateMaterialGroup,
    ) -> Result<ResponseMaterialGroup, LibError> {
        self.0.create_material_group(material_group).await
    }

    pub async fn get_all_material_groups(&self) -> Result<Vec<ResponseMaterialGroup>, LibError> {
        self.0.get_all_material_groups().await
    }

    pub async fn get_material_group_by_id(
        &self,
        id: Uuid,
    ) -> Result<ResponseMaterialGroup, LibError> {
        self.0.get_material_group_by_id(id).await
    }

    pub async fn get_sub_group_by_group_name(
        &self,
        group_name: &str,
    ) -> Result<Vec<ResponseMaterialGroup>, LibError> {
        self.0.get_sub_group_by_group_name(group_name).await
    }

    pub async fn delete_material_group_by_id(&self, id: &Uuid) -> Result<bool, LibError> {
        match self.0.delete_material_group_by_id(id).await {
            Ok(r) => match r {
                true => Ok(true),
                false => Err(LibError::SqlxError(sqlx::Error::RowNotFound)),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update_material_group_by_id(
        &self,
        id: &Uuid,
        material_group: &UpdateMaterialGroup,
    ) -> Result<ResponseMaterialGroup, LibError> {
        self.0.update_material_group_by_id(id, material_group).await
    }
}
