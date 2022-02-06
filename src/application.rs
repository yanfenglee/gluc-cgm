use actix_web::{HttpServer, App, web};
use anyhow::Result;
use log::info;

use crate::{DB, controller, settings::Settings};

pub async fn run() -> Result<(), anyhow::Error> {
    env_logger::init();

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