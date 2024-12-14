
//* use super for Parent Module Access: because all are within the routes module.
use super::util::{create_response, Response};
use axum::Json;

pub async fn get_categories() -> Json<Response> {
    println!("Categories");
    let resp = create_response(200, "Categories".to_string()).await;

    resp
}