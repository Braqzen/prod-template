use {
    crate::{
        api::{
            method::{MethodCaller, MethodHandler, MethodValidator},
            request::Request,
            response::{ErrorBody, INTERNAL_ERROR, INVALID_PARAMS},
        },
        server::State,
    },
    alloy::{
        consensus::TxEnvelope,
        hex,
        primitives::{Address, Bytes},
        rlp::Decodable,
    },
    serde_json::Value,
    std::sync::Arc,
    tracing::{error, info},
};

pub struct SendRawTransaction {
    state: Arc<State>,
    raw_transaction: String,
    envelope: TxEnvelope,
    signer: Address,
}

impl MethodValidator for SendRawTransaction {
    async fn validate(state: Arc<State>, request: &Request) -> Result<Self, ErrorBody> {
        let params = match request.params.as_array() {
            Some(arr) => arr,
            None => {
                return Err(ErrorBody::new(
                    INVALID_PARAMS,
                    "Invalid param type for eth_sendRawTransaction",
                ));
            }
        };

        let raw_transaction = match params.first() {
            Some(hex) => match hex.as_str() {
                Some(tx) => tx.to_string(),
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

        let transaction = match hex::decode(&raw_transaction) {
            Ok(bytes) => Bytes::from(bytes),
            Err(_) => {
                return Err(ErrorBody::new(INVALID_PARAMS, "Invalid transaction hex"));
            }
        };

        let envelope = TxEnvelope::decode(&mut transaction.as_ref())
            .map_err(|e| ErrorBody::new(INVALID_PARAMS, e.to_string()))?;

        let signer = match &envelope {
            TxEnvelope::Legacy(signed) => signed
                .recover_signer()
                .map_err(|e| ErrorBody::new(INVALID_PARAMS, e.to_string()))?,
            TxEnvelope::Eip2930(signed) => signed
                .recover_signer()
                .map_err(|e| ErrorBody::new(INVALID_PARAMS, e.to_string()))?,
            TxEnvelope::Eip1559(signed) => signed
                .recover_signer()
                .map_err(|e| ErrorBody::new(INVALID_PARAMS, e.to_string()))?,
            TxEnvelope::Eip4844(signed) => signed
                .recover_signer()
                .map_err(|e| ErrorBody::new(INVALID_PARAMS, e.to_string()))?,
            TxEnvelope::Eip7702(signed) => signed
                .recover_signer()
                .map_err(|e| ErrorBody::new(INVALID_PARAMS, e.to_string()))?,
        };

        Ok(Self {
            state,
            raw_transaction: raw_transaction.to_lowercase(),
            envelope,
            signer,
        })
    }
}

impl MethodCaller for SendRawTransaction {
    async fn call(self) -> Result<Value, ErrorBody> {
        let hash = self.envelope.hash().to_string().to_lowercase();
        let signer = self.signer.to_string().to_lowercase();

        // TODO: minor nuisance, logs "Internal server error" in caller instead of error
        self.state
            .database
            .create_transaction(
                hash.clone(),
                signer.clone(),
                self.raw_transaction,
                "Received".into(),
            )
            .await
            .map_err(|_| ErrorBody::new(INTERNAL_ERROR, "Internal server error"))?;

        info!(signer, hash, "Transaction created");

        if let Err(error) = self.state.database.notify().await {
            error!(%error, "Error sending notify");
        }

        Ok(Value::String(hash).into())
    }
}

impl From<SendRawTransaction> for MethodHandler {
    fn from(v: SendRawTransaction) -> Self {
        Self::SendRaw(v)
    }
}
