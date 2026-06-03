use crate::api::{Result, endpoint::ListPost};
use axum::{Json, extract::State};
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

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<ListPost>>> {
    let posts = sqlx::query_as!(
        ListPost,
        "SELECT id, title, created_at AS posted_at
        FROM posts
        WHERE published_at IS NULL
        ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(posts))
}
