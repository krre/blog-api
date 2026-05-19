use crate::api::{Error, Result, extract::AuthUser};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State};
use sqlx::PgPool;

pub(crate) mod router {
    use super::*;
    use axum::routing;
    use sqlx::{Pool, Postgres};

    pub fn new(pool: &Pool<Postgres>) -> routing::Router {
        routing::Router::new()
            .route("/", routing::get(get))
            .route("/", routing::post(update))
            .route("/password", routing::patch(update_password))
            .with_state(pool.clone())
    }
}

mod request {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct Profile {
        pub first_name: String,
        pub last_name: String,
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
    }
}

async fn get(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
) -> Result<Json<response::Profile>> {
    let user = sqlx::query_as!(
        response::Profile,
        "SELECT username, first_name, last_name FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(user))
}

pub async fn update(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    payload: axum::extract::Json<request::Profile>,
) -> Result<()> {
    sqlx::query!(
        "UPDATE users SET first_name = $1, last_name = $2, updated_at = current_timestamp WHERE id = $3",
        payload.first_name,
        payload.last_name,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(())
}

pub async fn update_password(
    State(pool): State<PgPool>,
    AuthUser(user_id): AuthUser,
    payload: axum::extract::Json<request::Password>,
) -> Result<()> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| Error::InternalServerError(format!("cannot hash password: {}", e)))?
        .to_string();

    sqlx::query!(
        "UPDATE users SET password_hash = $1, updated_at = current_timestamp WHERE id = $2",
        password_hash,
        user_id
    )
    .execute(&pool)
    .await?;

    Ok(())
}
