use std::fmt::{Debug};

use actix_web::{HttpResponse, body::BoxBody};

use crate::Ret;


#[derive(thiserror::Error, Debug)]
pub enum GlucError {
    #[error("请重新登陆: {0}")]
    AuthError(String),

    #[error("{0}")]
    RegisterError(String),

    #[error("{0}")]
    DBError(#[from] mongodb::error::Error),

    #[error("{0}")]
    UnknownError(String)
}

impl GlucError {
    pub fn get_code(&self) -> &'static str {
        match self {
            Self::AuthError(..) => "1001",
            Self::RegisterError(..) => "1002",
            Self::DBError(..) => "1003",
            Self::UnknownError(..) => "1004",
        }
    }
}

impl actix_web::ResponseError for GlucError {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        let ret: Ret<()> = self.into();
        HttpResponse::Ok().status(self.status_code()).json(ret)
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }
}

