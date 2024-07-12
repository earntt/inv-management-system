use api::rest::auth::handler::auth_handler;
use api::rest::material::handler::material_handler;
use api::rest::middleware::auth::{AuthLayer, RoleCheckLayer};
use api::rest::role::handler::role_handler;
use api::rest::supplier::handler::supplier_handler;
use api::rest::user::handler::user_handler;
use api::AppState;
use axum::{routing::get, Extension, Json, Router};
use config::Config;
use lib::util::postgres::get_connection_pool;
use lib::{
    app_ctx::AppCtx, auth::auth::Auth, material::material::PgMaterialRepository,
    role::role::PgRoleRepository, supplier::supplier::PgSupplierRepository,
    user::user::PgUserRepository,
};
use sqlx::postgres::Postgres;
use sqlx::Pool;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let config = Config::default();
    let pool: Arc<Pool<Postgres>> = Arc::new(get_connection_pool(config.clone()));
    let user_repository = Box::new(PgUserRepository::new(pool.clone()).await);
    let role_repository = Box::new(PgRoleRepository::new(pool.clone()).await);
    let supplier_repository = Box::new(PgSupplierRepository::new(pool.clone()).await);
    let material_repository = Box::new(PgMaterialRepository::new(pool.clone()).await);
    let auth_repository = Box::new(Auth::new().await);
    let app_ctx: AppCtx = AppCtx::new(
        user_repository,
        role_repository,
        supplier_repository,
        material_repository,
        auth_repository,
    )
    .await;
    let arc_state: Arc<AppCtx> = Arc::new(app_ctx);
    let app_state = Arc::new(AppState {
        arc_state: arc_state.clone(),
        config: config.clone(),
    });

    let public_api: Router = Router::new()
        .route("/", get(root))
        .nest("/auth", auth_handler());

    let api = Router::new()
        .nest("/users", user_handler())
        .nest("/roles", role_handler())
        .nest("/materials", material_handler())
        .nest("/suppliers", supplier_handler())
        .layer(AuthLayer::new(app_state.clone()));

    let app = Router::new()
        .nest("/api", api)
        .nest("/api", public_api)
        .layer(Extension(arc_state.clone()))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind(&format!("0.0.0.0:{}", config.port))
        .await
        .expect("Tcp listener must be valid!");

    axum::serve(listener, app)
        .await
        .expect("Axum must be able to start");
}

async fn root() -> Json<&'static str> {
    Json("OK")
}
