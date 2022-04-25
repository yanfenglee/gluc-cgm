use actix_web::{HttpServer, App, web};
use anyhow::Result;
use tracing::{info, Level};
use tracing_subscriber;

use crate::{DB, controller, settings::Settings};

pub async fn run() -> Result<(), anyhow::Error> {
    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let number_of_yaks = 3;
    // this creates a new event, outside of any spans.
    info!(number_of_yaks, "preparing to shave yaks");

    // read config
    Settings::init()?;

    // init mongodb
    DB::init().await?;

    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 8))
            .configure(controller::user_controller::config)
            .configure(controller::cgm_controller::config)
    })
    .bind(&Settings::get().bind_addr)?
    .run()
    .await?;
    
    info!("application exit!!!");

    Ok(())
}