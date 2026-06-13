use crate::api::{Error, Result, argon2_hash, endpoint::JwtExt, extract::AuthUser, jwt};
use axum::{Extension, Json, extract::State};
use sqlx::PgPool;
use std::sync::Arc;

pub(crate) mod router {
    use super::*;
    use axum::routing::{Router, get, patch};
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> Router {
        Router::new()
            .route("/", get(get_one).post(update))
            .route("/password", patch(update_password))
            .with_state(pool.clone())
    }
}

mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Profile {
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub telegram: String,
        pub location: String,
        pub bio: String,
    }

    #[derive(Deserialize)]
    pub struct Password {
        pub password: String,
    }
}

mod response {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Profile {
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub email: String,
        pub telegram: String,
        pub location: String,
        pub bio: String,
    }

    #[derive(Serialize)]
    pub struct Token {
        pub token: String,
    }
}

async fn get_one(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<response::Profile>> {
    let user = sqlx::query_as!(
        response::Profile,
        "SELECT username, first_name, last_name, email, telegram, location, bio
        FROM users
        WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}

pub async fn update(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    jwt_ext: Extension<Arc<JwtExt>>,
    payload: axum::extract::Json<request::Profile>,
) -> Result<Json<response::Token>> {
    sqlx::query!(
        "UPDATE users
        SET first_name = $1, last_name = $2, email = $3, telegram = $4, location = $5, bio = $6, updated_at = current_timestamp WHERE id = $7",
        payload.first_name,
        payload.last_name,
        payload.email,
        payload.telegram,
        payload.location,
        payload.bio,
        user_id
    )
    .execute(&pool)
    .await?;

    let jwt_user = jwt::User {
        id: user_id,
        first_name: payload.first_name.clone(),
        last_name: payload.last_name.clone(),
    };

    let token = jwt::create_token(jwt_user, &jwt_ext.secret)
        .map_err(|e| Error::InternalServerError(format!("cannot create token: {}", e)))?;

    Ok(Json(response::Token { token }))
}

pub async fn update_password(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    jwt_ext: Extension<Arc<JwtExt>>,
    payload: axum::extract::Json<request::Password>,
) -> Result<Json<response::Token>> {
    let password_hash = argon2_hash::encode(&payload.password)?;

    sqlx::query!(
        "UPDATE users SET password_hash = $1, updated_at = current_timestamp WHERE id = $2",
        password_hash,
        user_id
    )
    .execute(&pool)
    .await?;

    let jwt_user = sqlx::query_as!(
        jwt::User,
        "SELECT id, first_name, last_name FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    let token = jwt::create_token(jwt_user, &jwt_ext.secret)
        .map_err(|e| Error::InternalServerError(format!("cannot create token: {}", e)))?;

    Ok(Json(response::Token { token }))
}
