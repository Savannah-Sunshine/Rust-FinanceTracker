use super::util::{create_response, Response};
use axum::Json;

pub async fn show_all() -> Json<Response> {
    let resp = create_response(200, "Show All".to_string()).await;

    resp
}