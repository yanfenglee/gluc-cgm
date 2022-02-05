use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Serialize, Deserialize};

use crate::db::CollectionName;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub _id: ObjectId,
    pub username: String,
    pub password: Option<String>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub token: String,
    pub create_time: DateTime,
}

impl CollectionName for User {
    fn name() -> &'static str {
        "User"
    }
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cgm {
    pub user_id: Option<ObjectId>,
    pub device: Option<String>,
    pub date: Option<i64>,
    pub dateString: Option<String>,
    pub sgv: Option<i32>,
    pub delta: Option<f32>,
    pub direction: Option<String>,
    #[serde(rename = "type")]
    pub type1: Option<String>,
    pub filtered: Option<f64>,
    pub unfiltered: Option<f64>,
    pub rssi: Option<i32>,
    pub noise: Option<i32>,
    pub sysTime: Option<String>,
    pub utcOffset: Option<i32>,
    pub slope: Option<f64>,
    pub intercept: Option<f64>,
    pub scale: Option<i32>,
    pub mbg: Option<f64>,
    pub create_time: Option<DateTime>,
}

impl CollectionName for Cgm {
    fn name() -> &'static str {
        "Cgm"
    }
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserDTO {
    pub token: String,
    pub username: String,
    pub nickname: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserLoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserRegisterDTO {
    pub username: String,
    pub password: String,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}