use {
    crate::{
        api::{
            ErrorBody,
            handlers::methods::{
                send_raw::SendRawTransaction, transaction_status::TransactionStatus,
            },
            method::{Method, MethodHandler, MethodValidator},
            request::Request,
            response::METHOD_NOT_FOUND,
        },
        server::State,
    },
    std::sync::Arc,
};

pub struct Validator;

impl Validator {
    pub async fn validate(
        state: Arc<State>,
        request: &Request,
    ) -> Result<MethodHandler, ErrorBody> {
        let method = Method::try_from(request.method.clone())
            .map_err(|error| ErrorBody::new(METHOD_NOT_FOUND, &error))?;

        Ok(match method {
            Method::SendRawTransaction => {
                SendRawTransaction::validate(state, request).await?.into()
            }
            Method::TransactionStatus => TransactionStatus::validate(state, request).await?.into(),
        })
    }
}
