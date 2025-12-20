use {
    crate::{
        api::{
            method::{MethodCaller, MethodHandler, MethodValidator},
            request::Request,
            response::{ErrorBody, INTERNAL_ERROR, INVALID_PARAMS},
        },
        server::State,
    },
    serde_json::Value,
    std::sync::Arc,
    tracing::info,
};

pub struct TransactionStatus {
    state: Arc<State>,
    hash: String,
}

impl MethodValidator for TransactionStatus {
    async fn validate(state: Arc<State>, request: &Request) -> Result<Self, ErrorBody> {
        let params = match request.params.as_array() {
            Some(arr) => arr,
            None => {
                return Err(ErrorBody::new(
                    INVALID_PARAMS,
                    "Invalid param type for project_transactionStatus",
                ));
            }
        };

        let hash = match params.first() {
            Some(hex) => match hex.as_str() {
                Some(hash) => hash.to_string(),
                None => {
                    return Err(ErrorBody::new(
                        INVALID_PARAMS,
                        "Invalid parameter type: expected string",
                    ));
                }
            },
            None => {
                return Err(ErrorBody::new(
                    INVALID_PARAMS,
                    "Invalid parameter type: empty array",
                ));
            }
        };

        Ok(Self {
            state,
            hash: hash.to_lowercase(),
        })
    }
}

impl MethodCaller for TransactionStatus {
    async fn call(self) -> Result<Value, ErrorBody> {
        // TODO: minor nuisance, logs "Internal server error" in caller instead of error
        let transaction = self
            .state
            .database
            .transaction(self.hash)
            .await
            .map_err(|_| ErrorBody::new(INTERNAL_ERROR, "Internal server error"))?;

        info!(
            signer = transaction.signer,
            hash = transaction.id,
            status = transaction.status,
            "Transaction found"
        );

        Ok(Value::String(transaction.status).into())
    }
}

impl From<TransactionStatus> for MethodHandler {
    fn from(v: TransactionStatus) -> Self {
        Self::TransactionStatus(v)
    }
}
