
use axum::response::IntoResponse;
use axum::{Json, Router};
//use axum::extract::Query;
use axum::routing::{post, get};
//use futures::TryStreamExt;
//use mongodb::bson::{doc, DateTime};
//use mongodb::options::FindOptions;

//use serde::Deserialize;
use serde_json::{Map, Value};

use crate::structs::{User};
use crate::Result;
//use crate::{DB};


pub fn route() -> Router {
    let app = Router::new()
        .route("/treatments", post(treatments_post).get(treatments_get))
        .route("/devicestatus", post(devicestatus))
        .route("/activity", post(activity).get(activity))
        .route("/status.json", get(get_status))
        .route("/profile", get(profile_get).post(profile_post));

    app
}

pub async fn treatments_post(_user: User, Json(data): Json<Map<String,Value>>) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! treatments_post: {:?}", data);

    Ok("1")
}

pub async fn devicestatus(_user: User, Json(data): Json<Map<String,Value>>) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! devicestatus: {:?}", data);

    Ok("1")
}

pub async fn activity(_user: User, Json( _data): Json<Map<String,Value>>) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! activity");

    Ok("1")
}

pub async fn treatments_get(_user: User) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! treatments_get");

    Ok(format!("Not Implemented treatments_get"))
}

pub async fn get_status(_user: User) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! get_status");

    Ok(format!("Not Implemented get_status"))
}

pub async fn profile_post(_user: User) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! profile_post");

    Ok(format!("Not Implemented profile_post"))
}

pub async fn profile_get(_user: User) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! profile_get");

    Ok(format!("Not Implemented profile_get"))
}
