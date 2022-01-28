#[macro_use]
extern crate log;


use gluc_cgm::service::CgmService;


use actix_web::{App, HttpResponse, HttpServer, web, get};


#[get("/")]
async fn index() -> HttpResponse {
    let _ = CgmService::ping().await;
    
    HttpResponse::Ok().json(r#"{"haha":"ok"}"#)
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
