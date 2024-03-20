use axum::{Extension, Json, Router, Server};
use axum::extract::Query;
// use axum::body::Body;
use axum::http::{StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use serde::{Deserialize, Serialize};
use serde_json::{json};
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

/// demo request body for "User"
/// {
///     "name": "John Wayne",
///     "phone_number": "98827635482"
/// }
#[derive(Deserialize)]
struct UserReq {
    name: String,
    phone_number: String,
}

#[derive(Serialize)]
struct UserResp {
    id: i32,
    name: String,
    phone_number: String,
}
async fn create_user(Extension(pool):Extension<MySqlPool>, Json(user):Json<UserReq>) -> impl IntoResponse {
    let result = match query("INSERT INTO demo (name, phone_number) VALUES (?, ?)")
        .bind(user.name)
        .bind(user.phone_number)
        .execute(&pool)
        .await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error while inserting data").into_response();
        }
    };

    if result.rows_affected() == 1 {
        (StatusCode::CREATED, "User created successfully").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error while inserting data").into_response()
    }
}

#[derive(Deserialize)]
struct UserUpdateReq {
    id: i32
}
async fn update_user(Extension(pool): Extension<MySqlPool>, Query(find): Query<UserUpdateReq>, Json(user): Json<UserReq>) -> impl IntoResponse {
    let result = match query("UPDATE demo SET name = ?, phone_number = ? WHERE id = ?")
        .bind(user.name)
        .bind(user.phone_number)
        .bind(find.id)
        .execute(&pool)
        .await {
        Ok(result) => result,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    };

    if result.rows_affected() >=1 {
        (StatusCode::CREATED, "User updated").into_response()
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error while updating data").into_response()
    }
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
        .route("/get",get(get_users))
        .route("/create",post(create_user))
        .route("/update",post(update_user))
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
