use crate::rest::middleware::auth::role_check;
use crate::rest::user::dto::{RequestCreateUserDto, RequestUpdateUserDto};
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
use lib::auth::model::AuthInfo;
use std::sync::Arc;
use uuid::Uuid;

fn user_handler_user() -> Router {
    Router::new()
        .route("/", post(create_user).put(update_user).delete(delete_user))
        .route("/info", get(get_info))
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin", "User"])
        }))
}

fn user_handler_admin() -> Router {
    Router::new()
        .route("/", get(get_users))
        .route(
            "/:id",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
        .route_layer(middleware::from_fn(|req, next| {
            role_check(req, next, vec!["Admin"])
        }))
}

pub fn user_handler() -> Router {
    Router::new()
        .merge(user_handler_user())
        .merge(user_handler_admin())
}

async fn create_user(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    ValidatedJson(_dto): ValidatedJson<RequestCreateUserDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .user_use_case
        .create_user(&_dto.to_create_user())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResult::from(
            result,
            "User created successfully".to_string(),
            StatusCode::CREATED.into(),
        )),
    ))
}

async fn update_user(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Extension(_user_info): Extension<AuthInfo>,
    path: Option<Path<Uuid>>,
    ValidatedJson(_dto): ValidatedJson<RequestUpdateUserDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let id = match path {
        Some(id) => id.0,
        None => _user_info.user_info.sub,
    };
    let result = _app_ctx
        .user_use_case
        .update_user(&id, &_dto.to_update_user())
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "User updated successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn delete_user(
    Extension(app_ctx): Extension<Arc<AppCtx>>,
    Extension(_user_info): Extension<AuthInfo>,
    path: Option<Path<Uuid>>,
) -> Result<impl IntoResponse, RestApiError> {
    let id = match path {
        Some(id) => id.0,
        None => _user_info.user_info.sub,
    };
    app_ctx
        .user_use_case
        .delete_user(&id)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            Null,
            "User deleted successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn get_user_by_id(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Path(_dto): Path<Uuid>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .user_use_case
        .get_user_by_id(&_dto)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "User found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn get_users(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
) -> Result<impl IntoResponse, RestApiError> {
    let result = _app_ctx
        .user_use_case
        .get_users()
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            result,
            "Users found".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}

async fn get_info(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    Extension(_user_info): Extension<AuthInfo>,
) -> Result<impl IntoResponse, RestApiError> {
    Ok((
        StatusCode::OK,
        Json(ApiResult::from(
            _user_info,
            "Get user info successfully".to_string(),
            StatusCode::OK.into(),
        )),
    ))
}
