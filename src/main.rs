// https://www.twilio.com/en-us/blog/build-high-performance-rest-apis-rust-axum

use axum::{Router, Server};
use axum::routing::{get, post};

#[tokio::main]
async fn main() {
    const PORT:i32  = 3008;
    // println!("Hello, world!");

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust" }))
        .route("/", post(|data| async { "POST Method" }))
        ;

    println!("Server running on port: {}", PORT);

    // let addr = format!("{}", PORT);
    // Server::bind(&"0.0.0.0:3000".parse().unwrap())
    Server::bind(&format!("0.0.0.0:{}", PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
