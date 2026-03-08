use axum::{Router, routing::get};

pub struct Application {}

impl Application {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new().route("/", get(handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        println!("listening on {}", listener.local_addr()?);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

async fn handler() -> &'static str {
    "Under construction"
}
