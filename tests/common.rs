use gluc_cgm::application;



pub async fn setup() -> Result<(), anyhow::Error>{
    application::run().await?;
    Ok(())
}