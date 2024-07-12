pub mod util {
    pub mod error;
    pub mod postgres;
}

pub mod user {
    pub mod model;
    pub mod repository;
    pub mod use_case;
    pub mod user;
}

pub mod role {
    pub mod entity;
    pub mod model;
    pub mod repository;
    pub mod role;
    pub mod use_case;
}

pub mod auth {
    pub mod auth;
    pub mod model;
    pub mod repository;
    pub mod use_case;
}

pub mod supplier {
    pub mod model;
    pub mod repository;
    pub mod supplier;
    pub mod use_case;
}

pub mod material {
    pub mod material;
    pub mod model;
    pub mod repository;
    pub mod use_case;
}

pub mod app_ctx;
