
use actix_web::{HttpResponse, Responder, body::{MessageBody, BoxBody}, web::{self, JsonBody}};
use error::GlucError;
use mongodb::{Database};
use once_cell::sync::OnceCell;
use serde::{Serialize, Deserialize};


pub mod service;
pub mod error;
pub mod structs;
pub mod controller;
pub mod util;
pub mod middleware;

// mongodb singleton
pub static MONGO: OnceCell<Database> = OnceCell::new();


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ret<T> {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Ret<T> {
    pub fn new(data: T) -> Self {
        Ret {
            code: "OK".into(),
            msg: None,
            data: Some(data)
        }
    }
}

pub fn ret<T>(data: T) -> Result<Ret<T>, GlucError> where T: Serialize {
    Ok(Ret::new(data))
}

// impl<T> Into<HttpResponse> for Ret<T>  where T: Serialize {
//     fn into(self) -> HttpResponse {
//         HttpResponse::Ok().json(self)
//     }
// }

impl<T> Responder for Ret<T>  where T: Serialize {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
        //web::Json::<T>(self).respond_to(req)
        HttpResponse::Ok().json(self)
    }
}

impl From<&GlucError> for Ret<()> {
    fn from(err: &GlucError) -> Ret<()> {
        let code_str: &'static str = err.into();
        Ret {
            code: code_str.into(),
            msg: Some(format!("gluc exception: {:?}", err)),
            data: None,
        }
    }
}