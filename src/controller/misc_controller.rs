
use axum::extract::RawBody;
use axum::response::IntoResponse;
use axum::routing::{post, get};
use axum::{Json, Router};
use futures::TryStreamExt;
use hyper::Body;
use mongodb::bson::{Document, Bson, doc, DateTime};
use mongodb::options::FindOptions;
use serde_json::{Map, Value};
use crate::structs::{User};
use crate::{Result, DB};


pub fn route() -> Router {
    let app = Router::new()
        .route("/treatments", post(treatments_post).put(treatments_post).get(treatments_get))
        .route("/devicestatus", post(devicestatus))
        .route("/activity", post(activity).get(activity))
        .route("/status.json", get(get_status))
        .route("/profile", get(profile_get).post(profile_post));

    app
}


async fn body2doc(body: Body) -> Result<Document> {
    let bytes = hyper::body::to_bytes(body).await?;
    let s = std::str::from_utf8(&bytes)?;
    let doc: Document = serde_json::from_str(s)?;
    Ok(doc)
}

pub async fn treatments_post(user: User, RawBody(body): RawBody) -> Result<impl IntoResponse> {

    let mut d = body2doc(body).await?;

    d.insert("user_id", user._id);
    d.insert("create_time", DateTime::now());
    
    DB::get().collection("Treatments").insert_one(d, None).await?;

    Ok("1")
}


pub async fn devicestatus(user: User, RawBody(body): RawBody) -> Result<impl IntoResponse> {
    let mut doc = body2doc(body).await?;
    
    doc.insert("user_id", user._id);
    doc.insert("create_time", DateTime::now());

    DB::get().collection("Devices").insert_one(doc, None).await?;

    Ok("1")
}

pub async fn activity(_user: User, Json( _data): Json<Map<String,Value>>) -> Result<impl IntoResponse> {
    tracing::debug!("not implemented!!! activity");

    Ok("1")
}

pub async fn treatments_get(user: User) -> Result<impl IntoResponse> {
    tracing::debug!("get treatments_get call: {:?}", user._id);

    let opt = FindOptions::builder()
    .sort(doc! {"timestamp": -1})
    .limit(10)
    .build();

    let twobefore = chrono::Utc::now().timestamp_millis() - 2*24*60*60*1000;
    let res: Vec<Bson> = DB::get().collection("Treatments")
        .find(
            doc! {"user_id": user._id, "timestamp": {"$gte": twobefore}},
            Some(opt),
        )
        .await?
        .try_collect()
        .await?;

    Ok(serde_json::to_string(&res)?)
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
