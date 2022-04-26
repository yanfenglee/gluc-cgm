use actix_web::{web, App, HttpServer};
use anyhow::Result;

use crate::{controller, settings::Settings, DB};

pub async fn run() -> Result<(), anyhow::Error> {
    // read config
    Settings::init()?;

    // init mongodb
    DB::init().await?;

    tracing::info!("start app...");

    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 8))
            .configure(controller::user_controller::config)
            .configure(controller::cgm_controller::config)
    })
    .bind(&Settings::get().bind_addr)?
    .run()
    .await?;

    tracing::info!("application exit!!!");

    Ok(())
}
