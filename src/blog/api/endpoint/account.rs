use crate::api::Result;
use axum::{Json, extract::State};
use serde::Serialize;
use sqlx::PgPool;

pub(crate) mod router {
    use axum::routing::{self, get};
    use sqlx::{Pool, Postgres};

    use crate::api::endpoint::account::get_one;

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/", get(get_one))
            .with_state(pool.clone())
    }
}

#[derive(Serialize)]
struct User {
    pub login: String,
    pub name: String,
}

async fn get_one(State(pool): State<PgPool>) -> Result<Json<User>> {
    let user = sqlx::query_as!(User, "SELECT login, name FROM users WHERE id = 1")
        .fetch_one(&pool)
        .await
        .unwrap();

    Ok(Json(user))
}
