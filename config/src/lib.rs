use dotenvy::dotenv;
use jsonwebtoken::{DecodingKey, EncodingKey};
use once_cell::sync::Lazy;

#[derive(Debug, Clone)]
pub struct Config {
    pub port: u16,
    pub pg_connection: String,
    pub connection_pool_size: u32,
}

#[derive(Clone)]
pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    dotenv().expect("Env loader must be valid!");
    let secret = std::env::var("JWT_SECRET").unwrap_or(String::from("secret"));
    Keys::new(secret.as_bytes())
});

impl Default for Keys {
    fn default() -> Self {
        KEYS.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        dotenv().expect("Env loader must be valid!");
        Self {
            port: std::env::var("PORT")
                .unwrap_or(String::from("3000"))
                .parse::<u16>()
                .expect("PORT must be valid u16"),
            pg_connection: std::env::var("DATABASE_URL").unwrap_or(String::from(
                "postgres://postgres:root@localhost:5432/default",
            )),
            connection_pool_size: std::env::var("CONNECTION_POOL_SIZE")
                .unwrap_or(String::from("10"))
                .parse::<u32>()
                .expect("CONNECTION_POOL_SIZE must be valid"),
        }
    }
}
