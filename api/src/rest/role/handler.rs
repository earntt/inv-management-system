use crate::rest::middleware::auth::role_check;
use crate::rest::role::dto::RequestRoleDto;
use crate::util::error::RestApiError;
use crate::util::res::{ApiResult, Null};
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

pub fn role_handler() -> Router {
    Router::new()
        .route("/", post(create_role).get(get_roles))
        .route(
            "/:id",
            get(get_role_by_id).put(update_role).delete(delete_role),
        )
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin"])
        }))
}

async fn create_role(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    ValidatedJson(_dto): ValidatedJson<RequestRoleDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .role_use_case
        .create_role(&_dto.to_request_role())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResult::from(
            result,
            "Role created successfully".to_string(),
            StatusCode::CREATED.into(),
        )),
    ))
}

async fn get_roles(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .role_use_case
        .get_roles()
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Roles found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn update_role(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_id): Path<Uuid>,
    ValidatedJson(_dto): ValidatedJson<RequestRoleDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .role_use_case
        .update_role(&_id, &_dto.to_request_role())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Role updated successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn delete_role(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_id): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    _app_ctx
        .role_use_case
        .delete_role(&_id)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            Null,
            "Role deleted successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn get_role_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_id): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .role_use_case
        .get_role_by_id(&_id)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Role found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}
