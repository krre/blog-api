use crate::api::Result;
use axum::{Json, extract::State};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing;
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/", routing::get(get_one))
            .with_state(pool.clone())
    }
}

mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct User {
        pub login: String,
        pub name: String,
    }
}

async fn get_one(State(pool): State<PgPool>) -> Result<Json<response::User>> {
    let user = sqlx::query_as!(response::User, "SELECT login, name FROM users WHERE id = 1")
        .fetch_one(&pool)
        .await?;

    Ok(Json(user))
}
