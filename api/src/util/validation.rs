use crate::util::error::RestApiError;
use async_trait::async_trait;
use axum::{
    extract::{rejection::FormRejection, Form, FromRequest, FromRequestParts, Query, Request},
    http::request::Parts,
    response::Json,
    RequestExt,
};
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedQuery<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for ValidatedQuery<T>
where
    T: DeserializeOwned + Validate + std::fmt::Debug,
    S: Send + Sync,
{
    type Rejection = RestApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let Query(value) = Query::<T>::from_request_parts(parts, state).await?;
        value.validate()?;
        Ok(ValidatedQuery(value))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<J>(pub J);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + 'static,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = RestApiError;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = req.extract::<Json<T>, _>().await?;
        data.validate()?;
        Ok(Self(data))
    }
}
