use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use redis::{Commands, Connection, RedisResult};
use std::{thread, time};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Todoのデータ構造
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: Option<i64>,
    title: String,
    completed: bool,
}

#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
}
#[derive(Debug, Deserialize)]
struct TestRequest {
    key: String,
    value: String,
}

// アプリケーションの状態
type AppState = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main()-> RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut conn = client.get_connection()?;

    let state = Arc::new(Mutex::new(conn));

    // ルーティングの設定
    let app = Router::new()
        .route("/redis_add", post(redis_add))
        .with_state(state);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind");
    
    println!("Server running on http://localhost:8080");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");

    Ok(())
}

async fn redis_add(
    State(state): State<AppState>,
    Json(payload): Json<TestRequest>,
) -> (StatusCode, String) {
    //println!("Received request: {:?}", payload);
    let mut conn = state.lock().unwrap();
    let _: () = conn.set(&payload.key, &payload.value).unwrap();
    (StatusCode::CREATED, "OK".to_string())
}
