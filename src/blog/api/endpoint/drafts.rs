use crate::api::{
    Result,
    endpoint::{ListPost, Pagination, Posts},
};
use axum::{
    Json,
    extract::{Query, State},
};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing::{Router, get};
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> Router {
        Router::new()
            .route("/", get(get_all))
            .with_state(pool.clone())
    }
}

pub async fn get_all(
    pagination: Query<Pagination>,
    State(pool): State<PgPool>,
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
