use serde::{Deserialize, Serialize};
use axum::Json;

#[derive(Deserialize, Serialize)]
pub struct Response {
    status: u16,
    message: String,
}

pub async fn create_response(status_code: u16, message: String) -> Json<Response> {
    let resp = Response {
        status: status_code,
        message: message,
    };

    Json(resp)
}

pub async fn not_found() -> Json<Response> {
    create_response(404, "Not Found".to_string()).await
}