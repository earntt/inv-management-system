use crate::supplier::model::{CreateSupplier, ResponseSupplier, UpdateSupplier};
use crate::util::error::LibError;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait SupplierRepository: Send + Sync + std::fmt::Debug {
    async fn get_all_suppliers(&self) -> Result<Vec<ResponseSupplier>, LibError>;
    async fn create_supplier(&self, user: &CreateSupplier) -> Result<ResponseSupplier, LibError>;
    async fn get_supplier_by_id(&self, id: &Uuid) -> Result<ResponseSupplier, LibError>;
    async fn delete_supplier_by_id(&self, id: &Uuid) -> Result<bool, LibError>;
    async fn update_supplier_by_id(
        &self,
        id: &Uuid,
        user: &UpdateSupplier,
    ) -> Result<ResponseSupplier, LibError>;
}
