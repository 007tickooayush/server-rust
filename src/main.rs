use axum::{Extension, Json, Router, Server};
use axum::body::Body;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{MySqlPool, query, Row};

async fn get_users(Extension(pool):Extension<MySqlPool>) -> impl IntoResponse {
    let rows = match query("SELECT id, name, phone_number FROM demo")
        .fetch_all(&pool)
        .await {
        Ok(rows) => rows ,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error while fetching data").into_response();
        }
    };

    let users: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|row| {

            json!({
                "id":row.try_get::<i32, _>("id").unwrap_or_default(),
                "name":row.try_get::<String, _>("name").unwrap_or_default(),
                "phone_number":row.try_get::<String, _>("phone_number").unwrap_or_default()
            })
        }).collect();

    (StatusCode::OK, Json(users)).into_response()
}

#[tokio::main]
async fn main() {
    const PORT:i32  = 3008;

    let db_user = "root";
    let db_pass = "root";
    let db_host = "localhost:3307";
    let db_name = "test";
    let db_url = format!("mysql://{}:{}@{}/{}", db_user,db_pass,db_host,db_name);

    let db_pool = MySqlPool::connect(&db_url)
        .await
        .expect("Unable to connect to the database");
    println!("Database connected");

    let app = Router::new()
        .route("/", get(|| async { "Hello Rust" }))
        .route("/demo",get(get_users))
        .layer(Extension(db_pool))
        ;

    println!("Server running on port: {}", PORT);

    // let addr = format!("{}", PORT);
    // Server::bind(&"0.0.0.0:3000".parse().unwrap())
    Server::bind(&format!("0.0.0.0:{}", PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
