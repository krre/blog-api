use crate::api::{Error, Result};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing;
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/", routing::get(get_one))
            .route("/login", routing::post(login))
            .with_state(pool.clone())
    }
}

mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct User {
        pub login: String,
        pub password: String,
    }
}

mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct User {
        pub login: String,
        pub name: String,
    }

    #[derive(Serialize)]
    pub struct Token {
        pub token: String,
    }
}

async fn get_one(State(pool): State<PgPool>) -> Result<Json<response::User>> {
    let user = sqlx::query_as!(response::User, "SELECT login, name FROM users WHERE id = 1")
        .fetch_one(&pool)
        .await?;

    Ok(Json(user))
}

pub async fn login(
    State(pool): State<PgPool>,
    payload: axum::extract::Json<request::User>,
) -> Result<Json<response::Token>> {
    struct User {
        password: String,
    }

    let user = sqlx::query_as!(
        User,
        "SELECT password FROM users WHERE login = $1",
        payload.login,
    )
    .fetch_one(&pool)
    .await;

    match user {
        Ok(user) => {
            if user.password.is_empty() && !payload.password.is_empty() {
                return Err(Error::Unauthorized);
            }

            if user.password != payload.password {
                return Err(Error::Unauthorized);
            }

            let token = "test".to_string();
            return Ok(Json(response::Token { token }));
        }
        Err(error) => match error {
            sqlx::Error::RowNotFound => {
                return Err(Error::Unauthorized);
            }
            _ => {
                return Err(Error::DatabaseError(error));
            }
        },
    }
}
