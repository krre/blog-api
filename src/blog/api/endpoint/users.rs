use crate::api::Result;
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing;
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/{id}", routing::get(get_one))
            .with_state(pool.clone())
    }
}

mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct User {
        pub username: String,
        pub first_name: String,
        pub last_name: String,
    }
}

async fn get_one(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<Json<response::User>> {
    let user = sqlx::query_as!(
        response::User,
        "SELECT username, first_name, last_name FROM users WHERE id = $1",
        id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}
