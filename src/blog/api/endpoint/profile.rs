use crate::api::{Result, extract::AuthUser};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing;
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/", routing::get(get))
            .with_state(pool.clone())
    }
}

mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Profile {
        pub username: String,
        pub first_name: String,
        pub last_name: String,
    }
}

async fn get(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<response::Profile>> {
    let user = sqlx::query_as!(
        response::Profile,
        "SELECT username, first_name, last_name FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}
