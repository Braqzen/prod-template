use {
    crate::api::{ErrorBody, Response as ApiResponse},
    axum::Json,
    serde_json::Value,
};

// Handler for routes that don't exist
pub async fn not_found() -> Json<ApiResponse<Value>> {
    Json(ApiResponse::error(
        ErrorBody::new(-32600, "Invalid request endpoint"),
        Value::Number(0.into()),
    ))
}
