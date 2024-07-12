use axum::extract::rejection::{JsonRejection, QueryRejection};
use axum::{body::Body, http::Response, response::IntoResponse};
use lib::util::error::{DatabaseError, LibError};
use thiserror::Error;

use crate::util::res::{ApiResult, Null};

#[derive(Error, Debug)]
pub enum RestApiError {
    #[error("Bad request occured with message'{0}'")]
    BadRequest(String),
    #[error("Unautorized occured with message'{0}'")]
    Unauthorized(String),
    #[error("Bad request occured with message'{0}'")]
    Forbidden(String),
    #[error("Not found occured with message'{0}'")]
    NotFound(String),
    #[error("Bad request occured with message'{0}'")]
    InternalServerError(String),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
    #[error(transparent)]
    AxumQueryRejection(#[from] QueryRejection),
    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),
}

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Wrong credentials")]
    WrongCredentials,
    #[error("Missing credentials")]
    MissingCredentials,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Forbidden")]
    Forbidden,
}

impl RestApiError {
    pub fn from_lib(err: LibError) -> Self {
        match err {
            LibError::Database(err) => match err {
                DatabaseError::NotFound(_) => Self::NotFound("Not Found".to_string()),
                DatabaseError::Other(err) => Self::InternalServerError(err),
            },
            LibError::Http(err) => Self::InternalServerError(err),
            LibError::Storage(err) => Self::InternalServerError(err),
            LibError::SqlxError(err) => match err {
                sqlx::Error::RowNotFound => Self::NotFound("Not Found".to_string()),
                _ => Self::InternalServerError("Sqlx Error".to_string()),
            },
            LibError::NotFound(err) => Self::NotFound(err),
            LibError::BcryptError(err) => Self::InternalServerError(err.to_string()),
            LibError::Unauthorized(err) => Self::Unauthorized(err),
            LibError::JwtError(err) => Self::InternalServerError(err.to_string()),
        }
    }
    pub fn from_auth(err: AuthError) -> Self {
        match err {
            AuthError::WrongCredentials => Self::Unauthorized("Wrong credentials".to_string()),
            AuthError::MissingCredentials => Self::BadRequest("Missing credentials".to_string()),
            AuthError::InvalidToken => Self::BadRequest("Invalid token".to_string()),
            AuthError::Forbidden => Self::Forbidden("Forbidden".to_string()),
        }
    }
}

impl IntoResponse for RestApiError {
    fn into_response(self) -> Response<Body> {
        match self {
            RestApiError::BadRequest(msg) => create_response_json(400, msg),
            RestApiError::Unauthorized(msg) => create_response_json(401, msg),
            RestApiError::Forbidden(msg) => create_response_json(403, msg),
            RestApiError::NotFound(msg) => create_response_json(404, msg),
            RestApiError::InternalServerError(msg) => create_response_json(500, msg),
            RestApiError::ValidationError(_) => create_response_json(
                400,
                format!("Input validation error: [{self}]").replace('\n', ", "),
            ),
            RestApiError::AxumJsonRejection(_) => create_response_json(400, self.to_string()),
            RestApiError::AxumQueryRejection(e) => create_response_json(400, e.to_string()),
        }
    }
}

fn create_response_json(code: u16, msg: String) -> Response<Body> {
    let response = Response::builder().header("Content-Type", "application/json");
    let body = serde_json::to_string(&ApiResult::from(Null, msg, code))
        .expect("This serialize must be valid");
    response
        .status(code)
        .body(Body::new(body))
        .expect("Response must be valid")
}
