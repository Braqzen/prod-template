//! Middleware used to validate incoming HTTP requests.
//!
//! The layer implements a simple JSON-RPC validator which inspects the body and enforces deserialization.
//! The validator does not enforce anything within the body itself as long as it matches the structure
//! therefore "invalid" methods / parameters are allowed as long as deserialization is valid.
//!
//! The validator does not enforce a size on the body therefore usize::MAX number of bytes may be sent
//! which may be a problem for performance / DoS attacks.
//! Other layers may be used to enforce a size limit on the body.

use crate::api::{Request as ApiRequest, middleware::create_response};
use axum::{
    body::{Body, to_bytes},
    http::{Method, Request},
    response::Response,
};
use futures_util::future::BoxFuture;
use std::{
    convert::Infallible,
    task::{Context, Poll},
};
use tower::{Layer, Service};
use tracing::{error, warn};

#[derive(Clone)]
pub struct JsonValidationLayer;

impl<S> Layer<S> for JsonValidationLayer {
    type Service = JsonValidator<S>;
    fn layer(&self, inner: S) -> Self::Service {
        JsonValidator { inner }
    }
}

#[derive(Clone)]
pub struct JsonValidator<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for JsonValidator<S>
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

            if parts.method != Method::POST {
                // Forward non-POST requests without validation
                let request = Request::from_parts(parts, body);
                return inner.call(request).await;
            }

            let body = match to_bytes(body, usize::MAX).await {
                Ok(body) => body,
                Err(error) => {
                    warn!(%error, middleware = "JsonValidator", "Failed to read request body");
                    return Ok(create_response("Failed to read request body"));
                }
            };

            if let Ok(json_rpc) = serde_json::from_slice::<ApiRequest>(&body) {
                let mut request = Request::from_parts(parts, Body::from(body));

                // Insert deserialized type into extensions to save work in subsequent layers
                request.extensions_mut().insert(json_rpc);

                let response = match inner.call(request).await {
                    Ok(response) => response,
                    Err(error) => {
                        // Note: Inner service is trait bound to be infallible so this can never happen
                        error!(%error, middleware = "JsonValidator", "Failed to call inner service");
                        return Ok(create_response("Internal server error"));
                    }
                };
                // Note: we forward without modifying the response
                return Ok(response);
            }

            warn!("Request Validation: Invalid JSON-RPC request");
            Ok(create_response("Invalid JSON-RPC request"))
        })
    }
}
