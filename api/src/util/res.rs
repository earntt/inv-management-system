use axum::body::Body;
use serde::Serialize;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Null;

#[derive(Clone, Copy, Debug, Serialize)]
pub struct Result<T>
where
    T: Serialize,
{
    pub data: T,
}

impl<T> Result<T>
where
    T: Serialize,
{
    pub fn from_data(data: T) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ApiResult<T = Null>
where
    T: Serialize,
{
    pub rslt: Result<T>,
    pub status_message: String,
    pub status_code: u16,
}

impl<T> From<ApiResult<T>> for Body
where
    T: Serialize,
{
    fn from(api_result: ApiResult<T>) -> Self {
        let json = serde_json::to_string(&api_result).expect("converter must be valid.");
        Body::from(json)
    }
}

impl<T> ApiResult<T>
where
    T: Serialize,
{
    pub fn from(data: T, status_message: String, status_code: u16) -> Self {
        Self {
            rslt: Result::from_data(data),
            status_message,
            status_code,
        }
    }
}
