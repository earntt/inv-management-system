use crate::rest::auth::dto::RequestLoginDto;
use crate::rest::user::dto::RequestCreateUserDto;
use crate::util::error::RestApiError;
use crate::util::res::ApiResult;
use crate::util::validation::ValidatedJson;
use axum::{
    extract::Extension,
    http::StatusCode,
    response::IntoResponse,
    routing::{post, Router},
    Json,
};
use lib::app_ctx::AppCtx;
use std::sync::Arc;

pub fn auth_handler() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

async fn login(
    Extension(_app_ctx): Extension<Arc<AppCtx>>,
    ValidatedJson(_dto): ValidatedJson<RequestLoginDto>,
) -> Result<impl IntoResponse, RestApiError> {
    let (email, password) = &_dto.to_login();
    let result = _app_ctx
        .user_use_case
        .login(&email, &password)
        .await
        .map_err(RestApiError::from_lib)?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResult::from(
            result,
            "Logged in successful".to_string(),
            StatusCode::CREATED.into(),
        )),
    ))
}

async fn register(
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
