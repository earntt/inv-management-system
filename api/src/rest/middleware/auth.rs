use std::{
    sync::Arc,
    task::{Context, Poll},
};

use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use futures_util::future::BoxFuture;
use lib::auth::model::AuthInfo;
use tower::{Layer, Service};

use crate::util::res::{ApiResult, Null};
use crate::AppState;

// Authentification Middleware
#[derive(Clone)]
pub struct AuthLayer {
    state: Arc<AppState>,
}

impl AuthLayer {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
}

impl<S> Layer<S> for AuthLayer {
    type Service = AuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthMiddleware {
            inner,
            state: self.state.clone(),
        }
    }
}

#[derive(Clone)]
pub struct AuthMiddleware<S> {
    inner: S,
    state: Arc<AppState>,
}

impl<S> Service<Request> for AuthMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call<'a>(&'a mut self, mut request: Request) -> Self::Future {
        let clone = self.inner.clone();
        let use_case = self.state.arc_state.auth_use_case.clone();
        let mut srv = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let maybe_token = request.headers().get("Authorization");

            if let Some(token) = maybe_token {
                if let Ok(token_str) = token.to_str() {
                    if let Some(token_no_bearer) = token_str.strip_prefix("Bearer ") {
                        let result = use_case.get_info(token_no_bearer).await;
                        if let Ok(user_info) = result {
                            request.extensions_mut().insert(user_info);
                            return srv.call(request).await;
                        }
                    }
                }
            }
            Ok(Response::builder()
                .header("Content-Type", "application/json")
                .status(401)
                .body(
                    ApiResult::from(
                        Null,
                        "no credentials or invalid credentials".to_string(),
                        StatusCode::UNAUTHORIZED.as_u16(),
                    )
                    .into(),
                )
                .unwrap())
        })
    }
}

// Role Checking Middleware
#[derive(Clone)]
pub struct RoleCheckLayer {
    state: Arc<AppState>,
    required_roles: Vec<&'static str>,
}

impl RoleCheckLayer {
    pub fn new(state: Arc<AppState>, required_roles: Vec<&'static str>) -> Self {
        Self {
            state,
            required_roles,
        }
    }
}

impl<S> Layer<S> for RoleCheckLayer {
    type Service = RoleCheckMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RoleCheckMiddleware {
            inner,
            state: self.state.clone(),
            required_roles: self.required_roles.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RoleCheckMiddleware<S> {
    inner: S,
    state: Arc<AppState>,
    required_roles: Vec<&'static str>,
}
// service layer middleware
impl<S, ReqBody> Service<Request<ReqBody>> for RoleCheckMiddleware<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Send + Clone + 'static,
    S::Future: Send,
    ReqBody: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let inner_clone = self.inner.clone();
        let use_case = self.state.arc_state.auth_use_case.clone();
        let mut srv = std::mem::replace(&mut self.inner, inner_clone);
        let required_roles = self.required_roles.clone();

        Box::pin(async move {
            if let Some(auth_info) = req.extensions().get::<AuthInfo>() {
                let result = use_case
                    .role_check(required_roles, auth_info.user_info.roles.clone())
                    .await;
                if result.unwrap() {
                    return srv.call(req).await;
                }
            }
            Ok(Response::builder()
                .header("Content-Type", "application/json")
                .status(403)
                .body(
                    ApiResult::from(
                        Null,
                        "no permission".to_string(),
                        StatusCode::FORBIDDEN.as_u16(),
                    )
                    .into(),
                )
                .unwrap())
        })
    }
}
// route layer middleware
pub async fn role_check(
    req: Request,
    next: Next,
    required_roles: Vec<&'static str>,
) -> Result<Response, StatusCode> {
    let extensions = req.extensions();

    if let Some(auth_info) = extensions.get::<AuthInfo>() {
        if required_roles.contains(&auth_info.user_info.roles.as_str()) {
            return Ok(next.run(req).await);
        }
    }
    Ok(Response::builder()
        .header("Content-Type", "application/json")
        .status(403)
        .body(
            ApiResult::from(
                Null,
                "no permission".to_string(),
                StatusCode::FORBIDDEN.as_u16(),
            )
            .into(),
        )
        .unwrap())
}
