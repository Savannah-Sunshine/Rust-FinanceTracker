use axum::{routing::get,routing::post, Router, extract::State};
use std::env;
use dotenv::dotenv; //Loads environment variables from a .env file
    // , http::StatusCode}
use sqlx::PgPool;
mod routes;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let db = PgPool::connect(&database_url).await.unwrap();

    let app = Router::new()
        .route("/", get(|| async { "Hello new Axum Rust App!" }))
        .route("/categories", get(routes::categories::get_categories))
        .route("/details", get(routes::details::get_details))
        .route("/show", get(routes::show::show_all))
        .route("/add", post(routes::add_financial::add))
        .fallback(get(routes::util::not_found));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Server running on port 8080");
    axum::serve(listener, app).await.unwrap();
}


