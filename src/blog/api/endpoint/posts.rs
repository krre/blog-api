use crate::api::{Result, extract::AuthUser};
use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing::{Router, get};
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> Router {
        Router::new()
            .route("/", get(get_all).post(create))
            .route("/{id}", get(get_one).put(update).delete(delete))
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
    pub struct ListPost {
        pub id: i64,
        pub title: String,
        #[serde(with = "time::serde::rfc3339::option")]
        pub published_at: Option<OffsetDateTime>,
    }

    #[derive(Serialize)]
    pub struct Post {
        pub id: i64,
        pub title: String,
        pub post: String,
        pub is_published: bool,
        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
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

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<response::ListPost>>> {
    let posts = sqlx::query_as!(
        response::ListPost,
        "SELECT id, title, published_at
        FROM posts
        WHERE is_published = true
        ORDER BY published_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(posts))
}

pub async fn get_one(
    Path(id): Path<i64>,
    State(pool): State<PgPool>,
) -> Result<Json<response::Post>> {
    let post = sqlx::query_as!(
        response::Post,
        "SELECT id, title, post, is_published, created_at, updated_at
        FROM posts
        WHERE id = $1",
        id,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(post))
}

pub async fn delete(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<()> {
    sqlx::query!("DELETE FROM posts WHERE id = $1", id)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn update(
    Path(id): Path<i64>,
    State(pool): State<PgPool>,
    payload: axum::extract::Json<request::Post>,
) -> Result<()> {
    sqlx::query!(
        "UPDATE posts SET title = $1, post = $2, is_published = $3, updated_at = current_timestamp WHERE id = $4",
        payload.title,
        payload.post,
        payload.is_published,
        id,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
