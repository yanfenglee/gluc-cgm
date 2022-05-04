use std::fmt::{Debug};


use axum::response::IntoResponse;
use http::header::ToStrError;

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
    AxumError(#[from] axum::Error),

    #[error("{0}")]
    ToStrErr(#[from] ToStrError),

    #[error("{0}")]
    UnknownError(String)
}

impl GlucError {
    pub fn get_code(&self) -> &'static str {
        match self {
            Self::AuthError(..) => "1001",
            Self::RegisterError(..) => "1002",
            Self::DBError(..) => "1003",
            Self::AxumError(..) => "1004",
            Self::ToStrErr(..) => "1005",
            Self::UnknownError(..) => "1006",
        }
    }
}

impl IntoResponse for GlucError {
    fn into_response(self) -> axum::response::Response {
        Ret::from(&self).into_response()
    }
}

