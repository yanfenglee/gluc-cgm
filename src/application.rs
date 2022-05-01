use actix_web::{web, App, HttpServer};
use anyhow::Result;

use crate::error::GlucError;
use crate::{controller, settings::Settings, DB};
use actix_web::http::{StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev};

fn add_error_header<B>(res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, actix_web::Error> {
    // res.response_mut().headers_mut().insert(
    //     header::CONTENT_TYPE,
    //     header::HeaderValue::from_static("Error"),
    // );

    tracing::warn!("error: 111111111");

    //Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))

    Err(actix_web::Error::from(GlucError::ActixError("asdf".into())))
}


pub async fn run() -> Result<(), anyhow::Error> {
    // read config
    Settings::init()?;

    // init mongodb
    DB::init().await?;

    tracing::info!("start app...");

    HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::BAD_REQUEST, add_error_header))
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
