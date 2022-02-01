use actix_web::{FromRequest, HttpRequest, http::{header::HeaderMap, Error}, web::{Query}, error::ParseError};
use mongodb::bson::doc;

use std::{pin::Pin, future::Future, collections::HashMap};

use crate::{MONGO, structs::User};

#[derive(Debug)]
pub struct AuthUser {
    pub(crate) user: User,
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
        let qs = Query::<HashMap<String, String>>::from_query(req.query_string()).unwrap();

        if let Some(token) = qs.get("token") {
            println!("from query string token: {}", token);
            return Self::from_token(&token.to_string()).await;
        } else {
            return Self::from_header(req.headers()).await;
        }
    }

    pub async fn from_token(token: &String) -> Option<Self> {
        let db = MONGO.get().unwrap();

        let user = db.collection::<User>("cgm").find_one(doc!{"token":token}, None).await.ok()??;
            
        Some(AuthUser { user })
    }
}

impl FromRequest for AuthUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self, Self::Error>>>>;

    fn from_request(
        req: &HttpRequest,
        _payload: &mut actix_web::dev::Payload,
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