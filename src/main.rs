#[macro_use]
extern crate log;


use gluc_cgm::{service::CgmService, error::GlucError, ret};


use actix_web::{App, HttpResponse, HttpServer, web, get};


#[get("/")]
async fn index() -> Result<HttpResponse, GlucError> {
    let data = CgmService::ping().await?;
    
    ret(data)
}

#[tokio::main]
async fn main() -> std::io::Result<()>{

    env_logger::init();

    info!("starting up");

    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(1024*1024*8))
            .service(index)
    })
        .bind("localhost:8999")?
        .run()
        .await

}
