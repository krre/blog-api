use blog::core::Application;
use tracing_subscriber::{EnvFilter, prelude::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(
            tracing_subscriber::fmt::layer()
                .without_time()
                .with_ansi(false),
        )
        .init();

    Application::new().run().await?;
    Ok(())
}
