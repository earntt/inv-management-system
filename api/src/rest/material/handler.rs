use crate::rest::material::dto::{RequestCreateMaterialGroupDto, RequestUpdateMaterialGroupDto};
use crate::rest::middleware::auth::role_check;
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

fn material_handler_admin() -> Router {
    Router::new()
        .route("/groups", post(create_material_group))
        .route(
            "/groups/:id",
            delete(delete_material_group_by_id).put(update_material_group_by_id),
        )
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin"])
        }))
}

fn material_handler_user() -> Router {
    Router::new()
        .route("/groups", get(get_all_material_groups))
        .route("/groups/:id", get(get_material_group_by_id))
        .route("/groups/sub/:group_name", get(get_sub_group_by_group_name))
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin", "User"])
        }))
}

pub fn material_handler() -> Router {
    Router::new()
        .merge(material_handler_user())
        .merge(material_handler_admin())
}

async fn create_material_group(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    ValidatedJson(_dto): ValidatedJson<RequestCreateMaterialGroupDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .material_use_case
        .create_material_group(&_dto.to_create_material_group())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResult::from(
            result,
            "Material group created successfully".to_string(),
            StatusCode::CREATED.into(),
        )),
    ))
}

async fn get_all_material_groups(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .material_use_case
        .get_all_material_groups()
        .await
        .map_err(RestApiError::from_lib)?;
    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Material groups found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn get_material_group_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .material_use_case
        .get_material_group_by_id(id)
        .await
        .map_err(RestApiError::from_lib)?;
    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Material group found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn get_sub_group_by_group_name(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(group_name): Path<String>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .material_use_case
        .get_sub_group_by_group_name(&group_name)
        .await
        .map_err(RestApiError::from_lib)?;
    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Sub groups found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn delete_material_group_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    _app_ctx
        .material_use_case
        .delete_material_group_by_id(&id)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            {},
            "Material Group deleted successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn update_material_group_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(id): Path<Uuid>,
    ValidatedJson(_dto): ValidatedJson<RequestUpdateMaterialGroupDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .material_use_case
        .update_material_group_by_id(&id, &_dto.to_update_material_group_by_id())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Material Group updated successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}
