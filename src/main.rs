#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};

use anyhow::Result;
use gluc_cgm::{DB, controller};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    // init mongodb
    DB::init().await?;

    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(1024 * 1024 * 8))
            .configure(controller::user_controller::config)
    })
    .bind("localhost:8999")?
    .run()
    .await?;
    
    info!("starting up");

    Ok(())
}
