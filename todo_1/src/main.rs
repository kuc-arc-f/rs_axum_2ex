use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
    Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Todoのデータ構造
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: Option<i64>,
    title: String,
    completed: bool,
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
    let conn = Connection::open("todos.db").expect("Failed to open database");
    
    // テーブル作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )",
        [],
    )
    .expect("Failed to create table");

    let state = Arc::new(Mutex::new(conn));

    // ルーティングの設定
    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/:id", delete(delete_todo))
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

// TODO一覧取得
async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let conn = state.lock().unwrap();
    let mut stmt = conn
        .prepare("SELECT id, title, completed FROM todos ORDER BY id DESC")
        .unwrap();
    
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .unwrap()
        .filter_map(|t| t.ok())
        .collect();
    
    Json(todos)
}

// TODO作成
async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> (StatusCode, Json<Todo>) {
    let conn = state.lock().unwrap();
    
    conn.execute(
        "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
        params![payload.title, false],
    )
    .unwrap();
    
    let id = conn.last_insert_rowid();
    
    let todo = Todo {
        id: Some(id),
        title: payload.title,
        completed: false,
    };
    
    (StatusCode::CREATED, Json(todo))
}

// TODO削除
async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> StatusCode {
    let conn = state.lock().unwrap();
    
    let rows_affected = conn
        .execute("DELETE FROM todos WHERE id = ?1", params![id])
        .unwrap();
    
    if rows_affected > 0 {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}