mod json_validation;
mod method_validation;

use crate::api::{ErrorBody, Response as ApiResponse};
use axum::{
    body::Body,
    http::{StatusCode, header},
    response::Response,
};
use serde_json::Value;
pub use {json_validation::JsonValidationLayer, method_validation::MethodValidationLayer};

pub fn create_response(message: &str) -> Response {
    let response = ApiResponse::<Value>::error(ErrorBody::new(-32600, message), Value::Null);

    let body = match serde_json::to_vec(&response) {
        Ok(body) => body,
        Err(_) => b"{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{\"code\":-32600,\"message\":\"Invalid JSON-RPC request\"}}".to_vec(),
    };

    // Hardcode the unwrap as a last effort in case the body contributed to the error
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap_or_else(|_| Response::new(Body::from(
            "{\"jsonrpc\":\"2.0\",\"id\":null,\"error\":{\"code\":-32600,\"message\":\"Invalid JSON-RPC request\"}}"
        )))
}
