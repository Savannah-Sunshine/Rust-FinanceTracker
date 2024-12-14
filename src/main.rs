use axum::{routing::get,routing::post, Router, extract::State};
    // , http::StatusCode}
// use sqlx::PgPool;
mod routes;


#[tokio::main]
async fn main() {
    // let db: PgPool = PgPool::connect("<your-db-connection-url-here>").await.unwrap();
    // let state = MyState { db };
    

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


