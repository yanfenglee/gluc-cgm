use actix_web::{web, get, post};
use futures::TryStreamExt;
use mongodb::bson::doc;
use mongodb::options::FindOptions;
//use chrono::NaiveDateTime;
//use rbatis::core::value::DateTimeNow;
use serde::Deserialize;

use crate::{Ret, DB, ret};
use crate::error::GlucError;
use crate::structs::Cgm;
use crate::middleware::auth_user::AuthUser;
use crate::middleware::auth;

/// config route service
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1")
        .wrap(auth::UserAuth)
        .service(receive_bg)
        .service(get_bg)
    );
}


/// receive bg
#[post("/entries")]
pub async fn receive_bg(user: AuthUser, arg: web::Json<Vec<Cgm>>) -> Result<Ret<()>, GlucError> {
    let mut data = arg.into_inner();

    let _ = data.iter_mut().map(|item| {
        item.user_id = Some(user.user.user_id.clone());
    });

    DB::coll().insert_many(data, None).await?;
    ret(())
}


#[derive(Debug, Deserialize)]
pub struct Info {
    count: i64,
    rr: i64,
}

#[get("/entries.json")]
pub async fn get_bg(user: AuthUser, info: web::Query<Info>) -> Result<Ret<Vec<Cgm>>, GlucError> {
    log::info!("query entries {:?}, {:?}", user, info);

    let opt = FindOptions::builder()
    .sort(doc!{"date": 1})
    .limit(info.count)
    .build();

    let res: Vec<Cgm> = DB::coll().find(doc!{"user_id": user.user.token, "date": {"$lte": info.rr}}, Some(opt)).await?.try_collect().await?;

    ret(res)
}


