use std::fmt::{Display, Debug};

use actix_web::{HttpResponse, body::BoxBody};
use strum_macros::{IntoStaticStr};

use crate::Ret;


#[derive(thiserror::Error, Debug, IntoStaticStr)]
pub enum GlucError {
    #[error("{0}")]
    AuthError(String),

    #[error("{0}")]
    DBError(mongodb::error::Error),

    #[error("{0}")]
    UnknownError(String)
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

impl From<mongodb::error::Error> for GlucError {
    fn from(err: mongodb::error::Error) -> Self {
        GlucError::DBError(err)
    }
}

// impl <E> From<E> for GlucError {
//     fn from(err: E) -> Self {
//         GlucError::UnknownError(err)
//     }
// }
