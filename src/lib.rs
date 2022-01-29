
use actix_web::HttpResponse;
use error::GlucError;
use serde::{Serialize, Deserialize};


#[macro_use]
extern crate lazy_static;
pub mod service;
pub mod error;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ret<T> {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Ret<T> {
    pub fn new(data: T) -> Self {
        Ret {
            code: "0000".into(),
            msg: None,
            data: Some(data)
        }
    }
}

pub fn ret<T>(data: T) -> Result<HttpResponse, GlucError> where T: Serialize {
    Ok(Ret::new(data).into())
}

impl<T> Into<HttpResponse> for Ret<T>  where T: Serialize {
    fn into(self) -> HttpResponse {
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