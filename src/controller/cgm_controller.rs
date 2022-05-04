
use axum::{Json, Router};
use axum::extract::Query;
use axum::routing::{post, get};
use futures::TryStreamExt;
use mongodb::bson::{doc, DateTime};
use mongodb::options::FindOptions;

use serde::Deserialize;

use crate::structs::{Cgm, User};
use crate::Result;
use crate::{ret, Ret, DB};


pub fn route() -> Router {
    let app = Router::new()
        .route("/entries", post(receive_bg))
        .route("/entries.json", get(get_bg));

    app
}

/// receive bg
pub async fn receive_bg(user: User, Json(mut data): Json<Vec<Cgm>>) -> Result<Ret<()>> {
    tracing::info!("receive cgm {:?}, {:?}", user, data);

    for item in data.iter_mut() {
        item.user_id = Some(user._id);
        item.create_time = Some(DateTime::now())
    }

    tracing::info!("store cgm: {:?}", data);

    DB::coll().insert_many(data, None).await?;
    ret(())
}

#[derive(Debug, Deserialize)]
pub struct Info {
    count: i64,
    rr: i64,
}

pub async fn get_bg(user: User, Query(info): Query<Info>) -> Result<Ret<Vec<Cgm>>> {
    tracing::info!("query entries {:?}, {:?}", user, info);

    let opt = FindOptions::builder()
        .sort(doc! {"_id": -1})
        .limit(info.count)
        .build();

    let res: Vec<Cgm> = DB::coll()
        .find(
            doc! {"user_id": user._id, "date": {"$lte": info.rr}},
            Some(opt),
        )
        .await?
        .try_collect()
        .await?;

    ret(res)
}
