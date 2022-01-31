use std::pin::Pin;
use std::task::{Context, Poll};

use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use futures::future::{ok, Ready};
use futures::Future;
use actix_web::dev::{Transform, Service};
use actix_http::error;
use crate::middleware::auth_user::AuthUser;
use std::cell::RefCell;
use std::rc::Rc;
use crate::base::resp::Resp;

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct UserAuth;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for UserAuth
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = MyAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MyAuthMiddleware { service: Rc::new(RefCell::new(service)) })
    }
}

pub struct MyAuthMiddleware<S> {
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for MyAuthMiddleware<S>
    where
        S: Service<Request=ServiceRequest, Response=ServiceResponse<B>, Error=Error> + 'static,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output=Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());

        let mut service = self.service.clone();

        let ret = async move {
            let (http_req, payload) = req.into_parts();

            if let Some(user) = AuthUser::from_request(&http_req).await {
                if let Ok(req) = ServiceRequest::from_parts(http_req, payload) {
                    let res = service.call(req).await?;

                    println!("Hi from response");

                    return Ok(res);
                }
            };

            let res = Resp {
                code: "1001".to_string(),
                msg: Some("账号未登录".to_string()),
                data: Some("")
            };

            let json_str = serde_json::to_string(&res).unwrap();
            return Err(error::ErrorUnauthorized(json_str));
        };


        Box::pin(ret)
    }
}