use crate::supplier::model::{CreateSupplier, ResponseSupplier, UpdateSupplier};
use crate::supplier::repository::SupplierRepository;
use crate::util::error::LibError;
use uuid::Uuid;

#[derive(Debug)]
pub struct SupplierUseCase(Box<dyn SupplierRepository>);

impl SupplierUseCase {
    pub fn new(repository: Box<dyn SupplierRepository>) -> Self {
        SupplierUseCase(repository)
    }

    pub async fn get_all_suppliers(&self) -> Result<Vec<ResponseSupplier>, LibError> {
        self.0.get_all_suppliers().await
    }

    pub async fn create_supplier(
        &self,
        supplier: &CreateSupplier,
    ) -> Result<ResponseSupplier, LibError> {
        self.0.create_supplier(supplier).await
    }

    pub async fn get_supplier_by_id(&self, id: &Uuid) -> Result<ResponseSupplier, LibError> {
        self.0.get_supplier_by_id(id).await
    }

    pub async fn delete_supplier_by_id(&self, id: &Uuid) -> Result<bool, LibError> {
        match self.0.delete_supplier_by_id(id).await {
            Ok(r) => match r {
                true => Ok(true),
                false => Err(LibError::SqlxError(sqlx::Error::RowNotFound)),
            },
            Err(e) => Err(e),
        }
    }

    pub async fn update_supplier_by_id(
        &self,
        id: &Uuid,
        supplier: &UpdateSupplier,
    ) -> Result<ResponseSupplier, LibError> {
        self.0.update_supplier_by_id(id, supplier).await
    }
}
