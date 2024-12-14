use super::util::{create_response, Response};
use axum::Json;

pub async fn get_details() -> Json<Response> {
    let resp = create_response(200, "Details".to_string()).await;
    resp
}