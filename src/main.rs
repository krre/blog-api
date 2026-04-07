use blog::core::Application;
use tracing_subscriber::{EnvFilter, prelude::*};

fn init_tracing() {
    let filter_layer = EnvFilter::from_default_env();
    let fmt_layer = tracing_subscriber::fmt::layer().without_time().json();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_tracing();

    Application::new().run().await?;
    Ok(())
}
