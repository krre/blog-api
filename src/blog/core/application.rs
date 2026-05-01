use crate::api::endpoint::Router;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Config {
    server_addr: String,
    database_url: String,
    jwt_secret: String,
}

pub struct Application {
    config: Config,
}

impl Config {
    fn from_env() -> Self {
        envy::from_env().expect("failed to load config")
    }
}

impl Application {
    pub fn new() -> Self {
        let config = Config::from_env();
        Self { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.config.database_url)
            .await?;

        sqlx::migrate!().run(&pool).await?;

        let listener = tokio::net::TcpListener::bind(&self.config.server_addr).await?;
        let router = Router::new(pool, &self.config.jwt_secret);

        info!("listening on http://{}", listener.local_addr()?);
        axum::serve(listener, router.into_make_service()).await?;
        Ok(())
    }
}
