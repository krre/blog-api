pub mod auth;
pub mod drafts;
pub mod posts;
pub mod profile;
pub mod users;

use axum::{Extension, middleware, routing::IntoMakeService};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
use time::OffsetDateTime;

use crate::api::middleware::console::log_request_response;

pub struct Router {
    axum_router: axum::Router,
}

pub struct JwtExt {
    pub secret: String,
}

#[derive(Serialize)]
pub struct ListPost {
    pub id: i64,
    pub title: String,
    #[serde(with = "time::serde::rfc3339::option")]
    pub posted_at: Option<OffsetDateTime>,
}

impl Router {
    pub fn new(pool: Pool<Postgres>, jwt_secret: &str) -> Self {
        let jwt_ext = Arc::new(JwtExt {
            secret: jwt_secret.to_owned(),
        });

        let router = axum::Router::new()
            .nest("/auth", auth::router::new(&pool))
            .nest("/profile", profile::router::new(&pool))
            .nest("/users", users::router::new(&pool))
            .nest("/posts", posts::router::new(&pool))
            .nest("/drafts", drafts::router::new(&pool))
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
