//! Middleware used to

use crate::api::{Request as ApiRequest, middleware::create_response};
use axum::{body::Body, http::Request, response::Response};
use futures_util::future::BoxFuture;
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{error, warn};

#[derive(Clone)]
pub struct MethodValidationLayer;

impl<S> Layer<S> for MethodValidationLayer {
    type Service = MethodValidator<S>;
    fn layer(&self, inner: S) -> Self::Service {
        MethodValidator { inner }
    }
}

#[derive(Clone)]
pub struct MethodValidator<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for MethodValidator<S>
where
    S: Service<Request<Body>, Response = Response, Error = Infallible> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = Infallible;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();

        Box::pin(async move {
            let (parts, body) = request.into_parts();

            let request = if let Some(request) = parts.extensions.get::<ApiRequest>() {
                // Add custom methods here
                if !["eth_sendRawTransaction", "project_transactionStatus"]
                    .contains(&request.method.as_str())
                {
                    warn!(method = request.method, "Unsupported method");

                    return Ok(create_response(&format!(
                        "Unsupported method: {}",
                        request.method
                    )));
                }

                Request::from_parts(parts, body)
            } else {
                error!("Method Validation: Server misconfiguration, missing json validation layer");
                return Ok(create_response("Internal server error"));
            };

            inner.call(request).await
        })
    }
}
