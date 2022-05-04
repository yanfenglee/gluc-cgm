use anyhow::Result;
use axum::handler::Handler;
use axum::response::IntoResponse;
use axum::Router;
use http::Uri;
use crate::error::GlucError;
use crate::{controller::cgm_controller, controller::user_controller, settings::Settings, DB};

async fn fallback(uri: Uri) -> impl IntoResponse {
    tracing::warn!("no route found {}", uri);
    GlucError::UnknownError(format!("No route for {}", uri))
}

pub async fn run() -> Result<(), anyhow::Error> {
    // read config
    Settings::init()?;

    // init mongodb
    DB::init().await?;

    tracing::info!("start app...");

    let app = Router::new()
        .nest("/user", user_controller::route())
        .nest("/api/v1", cgm_controller::route())
        .fallback(fallback.into_service());

    let addr = Settings::get().bind_addr.parse().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    tracing::info!("application exit!!!");

    Ok(())
}
