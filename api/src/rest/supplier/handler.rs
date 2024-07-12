use crate::rest::middleware::auth::role_check;
use crate::rest::supplier::dto::{RequestCreateSupplierDto, RequestUpdateSupplierDto};
use crate::util::error::RestApiError;
use crate::util::res::ApiResult;
use crate::util::validation::ValidatedJson;
use axum::{
    extract::{Extension, Path},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use lib::app_ctx::AppCtx;
use std::sync::Arc;
use uuid::Uuid;

fn supplier_handler_admin() -> Router {
    Router::new()
        .route("/", post(create_supplier))
        .route(
            "/:id",
            put(update_supplier_by_id).delete(delete_supplier_by_id),
        )
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin"])
        }))
}

fn supplier_handler_user() -> Router {
    Router::new()
        .route("/", get(get_all_suppliers))
        .route("/:id", get(get_supplier_by_id))
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin", "User"])
        }))
}

pub fn supplier_handler() -> Router {
    Router::new()
        .merge(supplier_handler_admin())
        .merge(supplier_handler_user())
}

async fn get_all_suppliers(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .supplier_use_case
        .get_all_suppliers()
        .await
        .map_err(RestApiError::from_lib)?;
    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Suppliers found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn create_supplier(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    ValidatedJson(_dto): ValidatedJson<RequestCreateSupplierDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .supplier_use_case
        .create_supplier(&_dto.to_create_supplier())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResult::from(
            result,
            "Supplier created successfully".to_string(),
            StatusCode::CREATED.into(),
        )),
    ))
}

async fn get_supplier_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_dto): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .supplier_use_case
        .get_supplier_by_id(&_dto)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Supplier found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn delete_supplier_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_dto): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    _app_ctx
        .supplier_use_case
        .delete_supplier_by_id(&_dto)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            {},
            "Supplier deleted successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn update_supplier_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_id): Path<Uuid>,
    ValidatedJson(_dto): ValidatedJson<RequestUpdateSupplierDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .supplier_use_case
        .update_supplier_by_id(&_id, &_dto.to_update_supplier())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Supplier updated successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}
