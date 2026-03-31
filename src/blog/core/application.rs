use axum::Router;
use serde::Deserialize;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct Config {
    server_addr: String,
}

pub struct Application {
    config: Config,
}

impl Config {
    fn from_env() -> Self {
        envy::from_env().expect("Failed to load config")
    }
}

impl Application {
    pub fn new() -> Self {
        let config = Config::from_env();

        Self { config }
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = tokio::net::TcpListener::bind(&self.config.server_addr).await?;
        let router = Router::new();

        info!("listening on http://{}", listener.local_addr()?);
        axum::serve(listener, router.into_make_service()).await?;
        Ok(())
    }
}
