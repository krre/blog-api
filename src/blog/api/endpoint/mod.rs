pub mod auth;
pub mod profile;

use axum::{Extension, middleware, routing::IntoMakeService};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

use crate::api::middleware::console::log_request_response;

pub struct Router {
    axum_router: axum::Router,
}

pub struct JwtExt {
    pub secret: String,
}

impl Router {
    pub fn new(pool: Pool<Postgres>, jwt_secret: &str) -> Self {
        let jwt_ext = Arc::new(JwtExt {
            secret: jwt_secret.to_owned(),
        });

        let router = axum::Router::new()
            .nest("/auth", auth::router::new(&pool))
            .nest("/profile", profile::router::new(&pool))
            .layer(Extension(jwt_ext))
            .layer(middleware::from_fn(log_request_response));

        Self {
            axum_router: router,
        }
    }

    pub fn into_make_service(self) -> IntoMakeService<axum::Router> {
        self.axum_router.into_make_service()
    }
}
