use crate::api::{Error, Result, endpoint::JwtExt, jwt};
use axum::{Extension, Json, extract::State};
use sqlx::PgPool;
use std::sync::Arc;

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
    jwt_ext: Extension<Arc<JwtExt>>,
    payload: axum::extract::Json<request::User>,
) -> Result<Json<response::Token>> {
    struct User {
        id: i64,
        password: String,
    }

    let user = sqlx::query_as!(
        User,
        "SELECT id, password FROM users WHERE login = $1",
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

            let token = jwt::create_token(user.id as i64, &jwt_ext.secret)
                .map_err(|e| Error::InternalServerError(format!("cannot create token: {}", e)))?;

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
