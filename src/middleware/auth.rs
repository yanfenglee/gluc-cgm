use std::{pin::Pin, future, fmt::Debug};

use actix_web::{dev::{Service, ServiceRequest, Transform}, HttpRequest};
use futures::future::{Ready, ready};


use crate::error::GlucError;

use super::auth_user::AuthUser;

//use crate::error::GlucError;


pub struct AuthService<S> where S: Service<ServiceRequest> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthService<S> where S: Service<ServiceRequest, Error = actix_web::Error> + 'static{
    type Response = S::Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn future::Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut core::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let qs = String::from(req.query_string());
        let headers = req.headers().clone();

        let fut = self.service.call(req);

        let res = async move {
            if let Some(user) = AuthUser::from_header_qs(headers, qs).await {
                let res = fut.await?;
                Ok(res)
            } else {
                println!("auth failed");
                Err(actix_web::Error::from(GlucError::AuthError("failed auth".to_string())))
            }
        };

        Box::pin(res)
        
    }
}

pub struct UserAuth;

impl<S> Transform<S, ServiceRequest> for UserAuth where S: Service<ServiceRequest, Error = actix_web::Error> + 'static {
    type Response = S::Response;
    type Error = S::Error;
    type InitError = ();
    type Transform = AuthService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;
    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthService {
            service,
        }))
    }
}