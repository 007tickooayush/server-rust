use axum::{Json, Router, Server};
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    email: String
}

// Function to Handle user creation
// /create-user
async fn create_user() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::CREATED)
        .body(Body::from("User created successfully"))
        .unwrap()
}

// Function to get list of users
// /users
async fn list_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: "Hellsent".to_string(),
            email: "hellsent@gmail.com".to_string()
        },
        User {
            id: 2,
            name: "Tracteon".to_string(),
            email: "tracteon@gmail.com".to_string()
        }
    ];

    Json(users)
}



#[tokio::main]
async fn main() {
    const PORT:i32  = 3008;
    // println!("Hello, world!");

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust" }))
        .route("/create-user", post(create_user))
        .route("/users",get(list_users))
        ;

    println!("Server running on port: {}", PORT);

    // let addr = format!("{}", PORT);
    // Server::bind(&"0.0.0.0:3000".parse().unwrap())
    Server::bind(&format!("0.0.0.0:{}", PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
