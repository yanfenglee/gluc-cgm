use gluc_cgm::application;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    application::run().await
}
