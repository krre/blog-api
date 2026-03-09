use axum::{Router, routing::get};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    host: String,
    port: u16,
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
        let app = Router::new().route("/", get(handler));
        let listener =
            tokio::net::TcpListener::bind(format!("{}:{}", self.config.host, self.config.port))
                .await?;
        println!("listening on {}", listener.local_addr()?);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn handler() -> &'static str {
    "Under construction"
}
