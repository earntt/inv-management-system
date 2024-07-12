use crate::auth::auth::Auth;
use crate::auth::use_case::AuthUseCase;
use crate::material::repository::MaterialRepository;
use crate::material::use_case::MaterialUseCase;
use crate::role::repository::RoleRepository;
use crate::role::use_case::RoleUseCase;
use crate::supplier::repository::SupplierRepository;
use crate::supplier::use_case::SupplierUseCase;
use crate::user::repository::UserRepository;
use crate::user::use_case::UserUseCase;

#[derive(Debug)]
pub struct AppCtx {
    pub user_use_case: UserUseCase,
    pub role_use_case: RoleUseCase,
    pub supplier_use_case: SupplierUseCase,
    pub material_use_case: MaterialUseCase,
    pub auth_use_case: AuthUseCase,
}

impl AppCtx {
    pub async fn new(
        user_approval_repository: Box<dyn UserRepository>,
        role_approval_repository: Box<dyn RoleRepository>,
        supplier_repository: Box<dyn SupplierRepository>,
        material_repository: Box<dyn MaterialRepository>,
        auth_approval_repository: Box<Auth>,
    ) -> AppCtx {
        let user_use_case = UserUseCase::new(user_approval_repository);
        let role_use_case = RoleUseCase::new(role_approval_repository);
        let supplier_use_case = SupplierUseCase::new(supplier_repository);
        let material_use_case = MaterialUseCase::new(material_repository);
        let auth_use_case = AuthUseCase::new(auth_approval_repository);
        AppCtx {
            user_use_case,
            role_use_case,
            supplier_use_case,
            material_use_case,
            auth_use_case,
        }
    }
}
