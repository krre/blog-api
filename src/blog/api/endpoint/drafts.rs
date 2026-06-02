use crate::api::Result;
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

mod response {
    use serde::Serialize;
    use time::OffsetDateTime;

    #[derive(Serialize)]
    pub struct Draft {
        pub id: i64,
        pub title: String,
        #[serde(with = "time::serde::rfc3339")]
        pub created_at: OffsetDateTime,
    }
}

pub async fn get_all(State(pool): State<PgPool>) -> Result<Json<Vec<response::Draft>>> {
    let posts = sqlx::query_as!(
        response::Draft,
        "SELECT id, title, created_at
        FROM posts
        WHERE is_published = false
        ORDER BY created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(posts))
}
