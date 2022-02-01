// use std::pin::Pin;
// use std::task::{Context, Poll};

// use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
// use futures::future::{ok, Ready};
// use futures::Future;
// use actix_web::dev::{Transform, Service};
// use crate::error::GlucError;
// use crate::middleware::auth_user::AuthUser;
// use std::cell::RefCell;
// use std::rc::Rc;


// pub struct Authware;

// impl<ServiceRequest> Service<ServiceRequest> for Authware
// {
//     type Response = ServiceResponse;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, req: ServiceRequest) -> Self::Future {
//         println!("Hi from start. You requested: {}", req.path());

//         let mut service = self.service.clone();

//         let ret = async move {
//             let (http_req, payload) = req.into_parts();

//             if let Some(user) = AuthUser::from_request(&http_req).await {
//                 if let Ok(req) = ServiceRequest::from_parts(http_req, payload) {
//                     let res = service.call(req).await?;

//                     println!("Hi from response");

//                     return Ok(res);
//                 }
//             };

//             return Err(GlucError::AuthError("账号未登陆".to_owned()));
//         };


//         Box::pin(ret)
//     }
// }