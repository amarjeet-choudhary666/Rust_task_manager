use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
    future::{ready, Ready},
    rc::Rc,
};

use crate::utils::jwt::verify_token;

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
    S::Future: 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);

        Box::pin(async move {
            // Extract the Authorization header
            let auth_header = req.headers().get("Authorization");

            if let Some(auth_header) = auth_header {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let token = &auth_str[7..]; // Remove "Bearer " prefix

                        match verify_token(token) {
                            Ok(claims) => {
                                // Store user ID in request extensions
                                req.extensions_mut().insert(claims.sub.clone());
                                let res = service.call(req).await?;
                                return Ok(res);
                            }
                            Err(_) => {
                                return Ok(req.into_response(
                                    HttpResponse::Unauthorized().body("Invalid token")
                                ));
                            }
                        }
                    }
                }
            }

            Ok(req.into_response(
                HttpResponse::Unauthorized().body("Authorization header missing or invalid")
            ))
        })
    }
}