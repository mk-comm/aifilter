use axum::response::IntoResponse;
use axum::Json;
use serde_json::json;

pub async fn index() -> impl IntoResponse {
    Json(json!({
    "message": "Hello World!"
    }))
}
