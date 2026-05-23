use crate::api::{Result, extract::AuthUser};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing;
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/", routing::post(create))
            .route("/", routing::get(get_all))
            .with_state(pool.clone())
    }
}

mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Post {
        pub title: String,
        pub post: String,
        pub is_published: bool,
    }
}

mod response {
    use serde::Serialize;
    use time::OffsetDateTime;

    #[derive(Serialize)]
    pub struct PostId {
        pub id: i64,
    }

    #[derive(Serialize)]
    pub struct Post {
        pub id: i64,
        pub title: String,
        pub post: String,
        pub is_published: bool,
        pub created_at: OffsetDateTime,
        pub updated_at: OffsetDateTime,
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    payload: axum::extract::Json<request::Post>,
) -> Result<Json<response::PostId>> {
    let post = sqlx::query_as!(
        response::PostId,
        "INSERT INTO posts (title, post, is_published, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, $4, current_timestamp, current_timestamp)
        RETURNING id",
        payload.title,
        payload.post,
        payload.is_published,
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(post))
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<response::Post>>> {
    let posts = sqlx::query_as!(
        response::Post,
        "SELECT id, title, post, is_published, created_at, updated_at
        FROM posts
        ORDER BY updated_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(posts))
}
