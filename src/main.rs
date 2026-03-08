use blog::core::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Application::new().run().await?;
    Ok(())
}
