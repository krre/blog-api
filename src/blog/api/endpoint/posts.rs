use crate::api::{
    Result,
    endpoint::{ListPost, Posts},
    extract::AuthUser,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing::{Router, get, patch};
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> Router {
        Router::new()
            .route("/", get(get_all).post(create))
            .route("/{id}", get(get_one).put(update).delete(delete))
            .route("/{id}/publish", patch(publish))
            .route("/{id}/hide", patch(hide))
            .with_state(pool.clone())
    }
}

mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Post {
        pub title: String,
        pub post: String,
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
        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339")]
        pub updated_at: OffsetDateTime,
        #[serde(with = "time::serde::rfc3339::option")]
        pub published_at: Option<OffsetDateTime>,
    }
}

pub async fn create(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    payload: axum::extract::Json<request::Post>,
) -> Result<Json<response::PostId>> {
    let post = sqlx::query_as!(
        response::PostId,
        "INSERT INTO posts (title, post, user_id, created_at, updated_at)
        VALUES ($1, $2, $3, current_timestamp, current_timestamp)
        RETURNING id",
        payload.title,
        payload.post,
        user_id,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(post))
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: i64,
    pub limit: i64,
}

pub async fn get_all(
    pagination: Query<Pagination>,
    State(pool): State<PgPool>,
) -> Result<Json<Posts>> {
    let posts = sqlx::query_as!(
        ListPost,
        "SELECT id, title, published_at AS posted_at
        FROM posts
        WHERE published_at IS NOT NULL
        ORDER BY published_at DESC
        OFFSET $1
        LIMIT $2",
        pagination.offset,
        pagination.limit
    )
    .fetch_all(&pool)
    .await?;

    let count_query = sqlx::query!(
        "SELECT count(*)
        FROM posts
        WHERE published_at IS NOT NULL"
    )
    .fetch_one(&pool)
    .await?;
    let count = count_query.count.unwrap_or(0);

    Ok(Json(Posts { posts, count }))
}

pub async fn get_one(
    Path(id): Path<i64>,
    State(pool): State<PgPool>,
) -> Result<Json<response::Post>> {
    let post = sqlx::query_as!(
        response::Post,
        "SELECT id, title, post, created_at, updated_at, published_at
        FROM posts
        WHERE id = $1",
        id,
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(post))
}

pub async fn publish(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<()> {
    sqlx::query!(
        "UPDATE posts
        SET published_at = current_timestamp
        WHERE id = $1",
        id
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn hide(Path(id): Path<i64>, State(pool): State<PgPool>) -> Result<()> {
    sqlx::query!(
        "UPDATE posts
        SET published_at = null
        WHERE id = $1",
        id
    )
    .execute(&pool)
    .await?;

    Ok(())
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
        "UPDATE posts
        SET title = $1, post = $2, updated_at = current_timestamp
        WHERE id = $3",
        payload.title,
        payload.post,
        id,
    )
    .execute(&pool)
    .await?;

    Ok(())
}
