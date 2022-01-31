use actix_web::{FromRequest, HttpRequest};
use actix_http::{Payload, Error};
use actix_http::error::ParseError;
use actix_http::http::HeaderMap;
use std::pin::Pin;
use futures::{Future};

use crate::util::local_cache;
use qstring::QString;

#[derive(Debug)]
pub struct AuthUser {
    pub(crate) user_id: i64,
    pub(crate) token: String,
}

impl AuthUser {
    pub async fn from_header(headers: &HeaderMap) -> Option<Self> {
        if let Some(header) = headers.get("token") {
            let token = header.to_str().unwrap().to_string();

            return Self::from_token(&token).await
        }

        None
    }

    pub async fn from_request(req: &HttpRequest) -> Option<Self> {
        let qs = QString::from(req.query_string());

        if let Some(token) = qs.get("token") {
            println!("from query string token: {}", token);
            return Self::from_token(&token.to_string()).await;
        } else {
            return Self::from_header(req.headers()).await;
        }
    }

    pub async fn from_token(token: &String) -> Option<Self> {
        if let Ok(Some(id)) = select_id(&token).await {
            // write cache
            local_cache::set(token.as_str(), &id);

            log::info!("cache miss, get from db: {}", id);

            return Some(AuthUser { user_id: id, token: token.clone() });
        }

        None
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(
        req: &HttpRequest,
        _payload: &mut Payload,
    ) -> Self::Future {

        let req = req.clone();

        let ret = async move {
            if let Some(user) = AuthUser::from_request(&req).await {
                Ok(user)
            } else {
                Err(Error::from(ParseError::Header))
            }
        };

        Box::pin(ret)
    }
}