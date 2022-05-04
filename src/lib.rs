
use std::fmt::{Debug};

use axum::{response::IntoResponse, Json};
use error::GlucError;
use serde::{Serialize, Deserialize};

pub type Result<V> = core::result::Result<V, GlucError>;

pub mod settings;
pub mod service;
pub mod error;
pub mod structs;
pub mod controller;
pub mod util;
pub mod middleware;

pub mod db;
pub use db::DB;
pub mod application;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ret<T> {
    pub code: String,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> Ret<T> {
    pub fn new(data: T) -> Self {
        Ret {
            code: "0".into(),
            msg: None,
            data: Some(data)
        }
    }
}

pub fn ret<T>(data: T) -> Result<Ret<T>> where T: Serialize {
    Ok(Ret::new(data))
}


impl<T> IntoResponse for Ret<T>  where T: Serialize {
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl From<&GlucError> for Ret<()>  {
    fn from(err: &GlucError) -> Ret<()> {
        Ret {
            code: err.get_code().into(),
            msg: Some(err.to_string()),
            data: None,
        }
    }
}