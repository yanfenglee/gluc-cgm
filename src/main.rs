#[macro_use]
extern crate log;

use gluc_cgm::{error::GlucError, ret, service::CgmService, MONGO, controller};

use actix_web::{get, web, App, HttpResponse, HttpServer};
use mongodb::{Client, options::ClientOptions};

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<(), GlucError> {
    env_logger::init();


    // init mongodb

    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Manually set an option.
    client_options.app_name = Some("My App".to_string());

    // Get a handle to the deployment.
    let db = Client::with_options(client_options)?.database("asdf");
    let _ = MONGO.set(db);

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
