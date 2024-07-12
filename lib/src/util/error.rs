use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibError {
    #[error("Database error occured with message '{0}'")]
    Database(DatabaseError),
    #[error("Storage error cccured with message '{0}'")]
    Storage(String),
    #[error("Sqlx error occured with message '{0}'")]
    SqlxError(#[from] sqlx::Error),
    #[error("API error with message '{0}'")]
    Http(String),
    #[error("Not found error occured with message '{0}'")]
    NotFound(String),
    #[error("Bcrypt error occured with message '{0}'")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("Unauthorized error occured with message '{0}'")]
    Unauthorized(String),
    #[error("Jwt error occured with message '{0}'")]
    JwtError(#[from] jsonwebtoken::errors::Error),
}

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Not found error occured with message '{0}'")]
    NotFound(String),
    #[error("Other error occured with message '{0}'")]
    Other(String),
}
