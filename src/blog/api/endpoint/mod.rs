pub mod account;

use axum::routing::{IntoMakeService, get};
use sqlx::{Pool, Postgres};

pub struct Router {
    axum_router: axum::Router,
}

impl Router {
    pub fn new(pool: Pool<Postgres>) -> Self {
        let router = axum::Router::new()
            .nest("/account", account::router::new(&pool))
            .route("/", get(handler));

        Self {
            axum_router: router,
        }
    }

    pub fn into_make_service(self) -> IntoMakeService<axum::Router> {
        self.axum_router.into_make_service()
    }
}

async fn handler() -> &'static str {
    "Under construction"
}
