//! This module provides types for responses as defined in the JSON-RPC 2.0 specification.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Response<T> {
    Success(Success<T>),
    Error(Error),
}

impl<T> Response<T> {
    pub fn success(result: T, id: Value) -> Self {
        Self::Success(Success::new(result, id))
    }

    pub fn error(error: ErrorBody, id: Value) -> Self {
        Self::Error(Error::new(error, id))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Success<T> {
    pub jsonrpc: String,
    pub result: T,
    pub id: Value,
}

impl<T> Success<T> {
    pub fn new(result: T, id: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result,
            id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub jsonrpc: String,
    pub error: ErrorBody,
    pub id: Value,
}

impl Error {
    pub fn new(error: ErrorBody, id: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            error,
            id,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ErrorBody {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl ErrorBody {
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    pub fn with_data(self, data: Value) -> Self {
        Self {
            data: Some(data),
            ..self
        }
    }
}

// https://www.jsonrpc.org/specification#error_object
pub const PARSE_ERROR: i32 = -32700;
pub const INVALID_REQUEST: i32 = -32600;
pub const METHOD_NOT_FOUND: i32 = -32601;
pub const INVALID_PARAMS: i32 = -32602;
pub const INTERNAL_ERROR: i32 = -32603;

// Custom domain-specific error codes (-32000 to -32099)
pub const NOT_FOUND: i32 = -32000;
