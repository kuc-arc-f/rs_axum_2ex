use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use anyhow::{ensure, Context, Result};
use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, PasswordHash, rand_core::OsRng};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::error::Error;

mod mod_auth;

// Todoのデータ構造
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: Option<i64>,
    title: String,
    completed: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct UserRes {
    email: String,
    password: String,
}
#[derive(Debug, Deserialize)]
struct UserCreateReq {
    email: String,
    password: String,
}

// Todo作成用のリクエスト
#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
}

// アプリケーションの状態
type AppState = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() {
    // SQLiteデータベースの初期化
    let conn = Connection::open("user.db").expect("Failed to open database");
    
    // テーブル作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            email TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )
    .expect("Failed to create table");

    let state = Arc::new(Mutex::new(conn));

    // ルーティングの設定
    let app = Router::new()
        .route("/signup", post(user_create))
        .route("/login", post(user_login))
        .with_state(state);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("Failed to bind");
    
    println!("Server running on http://localhost:8080");
    
    axum::serve(listener, app)
        .await
        .expect("Server failed to start");
}

async fn user_create(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateReq>,
) -> (StatusCode, Json<UserRes>) {
    let mut user = UserRes {
        email: "".to_string(),
        password: "".to_string(),
    };

    let conn = state.lock().unwrap();
    println!("{:?}", payload);

    let mut stmt = conn
        .prepare("SELECT id, email FROM users WHERE email = ?1")
        .unwrap();    
    let mut rows = stmt.query(params![payload.email]).unwrap();
    let mut user_ct = 0;
    while let Some(row) = rows.next().unwrap() {
        user_ct += 1;
    }
    println!("user_ct={}", user_ct);
    if user_ct > 0 {
        return (StatusCode::BAD_REQUEST , Json(user));
    }
    let pw = payload.password.clone();

    // get-hash
    let phc = mod_auth::hash_password(&pw).expect("error , hash_password");
    println!("phc: {}", phc);

    let ok = mod_auth::verify_password(&pw, &phc).expect("error , veryfy password");
    println!("password ok? {}", ok);    
    conn.execute(
        "INSERT INTO users (email, password_hash) VALUES (?1, ?2)",
        params![payload.email, phc],
    )
    .unwrap();
    let id = conn.last_insert_rowid();
    
    (StatusCode::CREATED, Json(user))
}

async fn user_login(
    State(state): State<AppState>,
    Json(payload): Json<UserCreateReq>,
) -> (StatusCode, Json<UserRes>) {    
    let user = UserRes {
        email: "".to_string(),
        password: "".to_string(),
    };
    let conn = state.lock().unwrap();
    // SELECT-User
    let mut stmt = conn
        .prepare("SELECT id, email , password_hash FROM users WHERE email = ?1")
        .unwrap();    
    let mut rows = stmt.query(params![payload.email]).unwrap();
    let mut user_ct = 0;
    let mut db_pass = "".to_string();
    while let Some(row) = rows.next().unwrap() {
        user_ct += 1;
        db_pass = row.get(2).unwrap();
    }
    println!("user_ct={}", user_ct);
    println!("db_pass={}", &db_pass);
    if user_ct == 0 {
        return (StatusCode::BAD_REQUEST , Json(user));
    }    

    println!("{:?}", payload);
    let pw = payload.password.clone();

    let phc = "".to_string();
    let phc = db_pass.clone();
    println!("phc: {}", phc);

    let ok = mod_auth::verify_password(&pw, &phc).expect("error , veryfy password");
    println!("password ok? {}", ok);    
    if ok == false {
        return (StatusCode::BAD_REQUEST, Json(user));
    }

    (StatusCode::OK, Json(user))
}

