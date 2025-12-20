use crate::{
    api::{Request, Response, validator::Validator},
    server::State as ServerState,
};
use axum::{Json, extract::State};
use serde_json::Value;
use std::sync::Arc;
use tracing::{info, warn};

#[axum::debug_handler]
pub async fn request(
    State(state): State<Arc<ServerState>>,
    Json(request): Json<Request>,
) -> Json<Response<Value>> {
    info!(
        method = request.method,
        params = ?request.params,
        id = %request.id,
        "Received request"
    );

    let method = match Validator::validate(state, &request).await {
        Ok(method) => method,
        Err(error) => {
            warn!(method = request.method, id = %request.id, message = error.message, "Invalid request");
            return Response::error(error, request.id).into();
        }
    };

    match method.call().await {
        Ok(result) => return Response::success(result, request.id).into(),
        Err(error) => {
            warn!(method = request.method, id = %request.id, message = error.message, "Error calling method");
            return Response::error(error, request.id).into();
        }
    }
}
