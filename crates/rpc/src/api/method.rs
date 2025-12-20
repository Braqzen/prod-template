use {
    crate::{
        api::{
            handlers::methods::{
                send_raw::SendRawTransaction, transaction_status::TransactionStatus,
            },
            request::Request,
            response::ErrorBody,
        },
        server::State,
    },
    serde_json::Value,
    std::sync::Arc,
};

pub enum Method {
    SendRawTransaction,
    TransactionStatus,
}

impl TryFrom<String> for Method {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "eth_sendRawTransaction" => Ok(Method::SendRawTransaction),
            "project_transactionStatus" => Ok(Method::TransactionStatus),
            _ => Err(format!("Invalid method: {value}")),
        }
    }
}

/// Trait for calling ETH JSON RPC methods
pub trait MethodCaller {
    async fn call(self) -> Result<Value, ErrorBody>;
}

/// Trait for validating ETH JSON RPC methods
pub trait MethodValidator: Sized {
    /// Validate the method
    async fn validate(state: Arc<State>, req: &Request) -> Result<Self, ErrorBody>;
}

/// Wrapper for a single unified return type that calls the underlying method
pub enum MethodHandler {
    SendRaw(SendRawTransaction),
    TransactionStatus(TransactionStatus),
}

impl MethodHandler {
    pub async fn call(self) -> Result<Value, ErrorBody> {
        match self {
            Self::SendRaw(method) => method.call().await,
            Self::TransactionStatus(method) => method.call().await,
        }
    }
}
