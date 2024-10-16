mod book;
mod serve;
mod search;
use axum::{routing::get, Router};
use search::*;
use serve::*;

#[tokio::main]
async fn main() {
    let ip: &str = "0.0.0.0";
    let port: &str = "8000";

    let combine: &str = &format!("{}:{}", ip, port);

    // Create the router with the routes
    let app = Router::new()
        .route("/get_tag", get(get_tag))
        .route("/search", get(search));

    // Define the address to run the server
    let addr = tokio::net::TcpListener::bind(combine).await.unwrap();

    println!("Server running at http://{}:{}", ip, port);
    test();

    axum::serve(addr, app).await.unwrap();
}
