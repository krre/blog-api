use crate::api::{
    Error, Result,
    endpoint::{ListPost, Pagination, Post, Posts},
    extract::AuthUser,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing::{Router, get};
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> Router {
        Router::new()
            .route("/", get(get_all))
            .route("/{id}", get(get_one))
            .with_state(pool.clone())
    }
}

pub async fn get_all(
    pagination: Query<Pagination>,
    State(pool): State<PgPool>,
    AuthUser(_): AuthUser,
) -> Result<Json<Posts>> {
    let posts = sqlx::query_as!(
        ListPost,
        "SELECT id, title, created_at AS posted_at
        FROM posts
        WHERE published_at IS NULL
        ORDER BY created_at DESC
        OFFSET $1
        LIMIT $2",
        pagination.offset,
        pagination.limit,
    )
    .fetch_all(&pool)
    .await?;

    let count_query = sqlx::query!(
        "SELECT count(*)
        FROM posts
        WHERE published_at IS NULL"
    )
    .fetch_one(&pool)
    .await?;
    let count = count_query.count.unwrap_or(0);

    Ok(Json(Posts { posts, count }))
}

pub async fn get_one(
    Path(id): Path<i64>,
    State(pool): State<PgPool>,
    AuthUser(_): AuthUser,
) -> Result<Json<Post>> {
    let result = sqlx::query_as!(
        Post,
        "SELECT id, title, post, created_at, updated_at, published_at
        FROM posts
        WHERE id = $1 AND published_at IS NULL",
        id,
    )
    .fetch_one(&pool)
    .await;

    match result {
        Ok(post) => Ok(Json(post)),
        Err(error) => match error {
            sqlx::Error::RowNotFound => Err(Error::NotFound),
            _ => Err(Error::DatabaseError(error)),
        },
    }
}
