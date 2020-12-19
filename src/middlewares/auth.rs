use std::pin::Pin;
use std::task::{Context, Poll};

use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, http::Method, HttpResponse};
use futures::future::{ok, Ready};
use futures::Future;

use crate::utils::jwt::{decode_token, verify_token};
use actix_web::web::Data;
use crate::AppState;

pub struct JwtAuth;

impl<S, B> Transform<S> for JwtAuth
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware { service })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: S,
}

impl<S, B> Service for JwtAuthMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut is_auth = false;
        let headers = req.headers();
        let state = req.app_data::<Data<AppState>>().expect("failed to get app state");
        let mut err_msg = String::from("unauthorized");

        if req.method().eq(&Method::OPTIONS) {
            is_auth = true;
        } else {
            if let Some(authorization_header) = headers.get("Authorization") {
                if let Ok(authorization_str) = authorization_header.to_str() {
                    if authorization_str.starts_with("bearer") ||
                        authorization_str.starts_with("Bearer") {
                        let token = authorization_str[6..authorization_str.len()].trim();
                        if let Ok(token_data) = decode_token(token.to_string()) {
                            let result = verify_token(token_data, &state);
                            match result {
                                Ok(_) => {
                                    is_auth = true;
                                },
                                Err(error) => {
                                    is_auth = false;
                                    err_msg = error;
                                },
                            };
                        }
                    }
                }
            }
        }

        if is_auth {
            let fut = self.service.call(req);

            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    HttpResponse::Unauthorized()
                        .json(json!({"error": err_msg}))
                        .into_body(),
                ))
            })
        }
    }
}