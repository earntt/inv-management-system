use config::Config;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::Postgres;
use sqlx::Pool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PgError {
    #[error("unexpected error with message: \"{msg:?}\"")]
    Error { msg: String },
}

impl PgError {
    pub fn msg(msg: String) -> Self {
        self::PgError::Error { msg }
    }
}

pub fn get_connection_pool(config: Config) -> Pool<Postgres> {
    let url: String = config.pg_connection;
    let connection_pool_size = config.connection_pool_size;
    PgPoolOptions::new()
        .max_connections(connection_pool_size)
        .connect_lazy(&url)
        .expect("PgPool Connection Error")
}
