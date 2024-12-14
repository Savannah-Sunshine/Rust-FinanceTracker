use super::util::{create_response, Response};
use axum::Json;

pub async fn add(Json(json): Json<String>,) -> Json<Response> {
    // format!("The contents of my_field is: {}", json)
    println!("The contents the input are: {}", json);

    let resp = create_response(200, "Add".to_string()).await;

    resp
}