use std::sync::Arc;

pub mod util {
    pub mod error;
    pub mod res;
    pub mod validation;
}
pub mod rest {
    pub mod user {
        pub mod dto;
        pub mod handler;
    }
    pub mod role {
        pub mod dto;
        pub mod handler;
    }
    pub mod supplier {
        pub mod dto;
        pub mod handler;
    }
    pub mod material {
        pub mod dto;
        pub mod handler;
    }
    pub mod auth {
        pub mod dto;
        pub mod handler;
    }
    pub mod middleware {
        pub mod auth;
    }
}

pub struct AppState {
    pub arc_state: Arc<lib::app_ctx::AppCtx>,
    pub config: config::Config,
}
