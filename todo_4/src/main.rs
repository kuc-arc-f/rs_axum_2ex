use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get},
    Router,
};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

// Todoのデータ構造 (table.sqlに対応)
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Todo {
    id: Option<i64>,
    title: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    is_public: bool,
    #[serde(default)]
    food_orange: bool,
    #[serde(default)]
    food_apple: bool,
    #[serde(default)]
    food_banana: bool,
    #[serde(default)]
    pub_date: Option<String>,
    #[serde(default)]
    qty1: i64,
    #[serde(default)]
    qty2: i64,
    #[serde(default)]
    qty3: i64,
}

// Todo作成用のリクエスト
#[derive(Debug, Deserialize)]
struct CreateTodoRequest {
    title: String,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    is_public: bool,
    #[serde(default)]
    food_orange: bool,
    #[serde(default)]
    food_apple: bool,
    #[serde(default)]
    food_banana: bool,
    #[serde(default)]
    pub_date: Option<String>,
    #[serde(default)]
    qty1: i64,
    #[serde(default)]
    qty2: i64,
    #[serde(default)]
    qty3: i64,
}

// アプリケーションの状態
type AppState = Arc<Mutex<Connection>>;

#[tokio::main]
async fn main() {
    // SQLiteデータベースの初期化
    let conn = Connection::open("todos.db").expect("Failed to open database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            content TEXT,
            is_public INTEGER,
            food_orange INTEGER,
            food_apple INTEGER,
            food_banana INTEGER,
            pub_date TEXT,
            qty1 INTEGER,
            qty2 INTEGER,
            qty3 INTEGER
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
        .prepare(
            "SELECT id, title, content, is_public, food_orange, food_apple, food_banana, pub_date, qty1, qty2, qty3 FROM todos ORDER BY id DESC",
        )
        .unwrap();
    
    let todos = stmt
        .query_map([], |row| {
            Ok(Todo {
                id: Some(row.get(0)?),
                title: row.get(1)?,
                content: row.get(2)?,
                is_public: row.get(3)?,
                food_orange: row.get(4)?,
                food_apple: row.get(5)?,
                food_banana: row.get(6)?,
                pub_date: row.get(7)?,
                qty1: row.get(8)?,
                qty2: row.get(9)?,
                qty3: row.get(10)?,
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
        "INSERT INTO todos (title, content, is_public, food_orange, food_apple, food_banana, pub_date, qty1, qty2, qty3)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            payload.title,
            payload.content,
            payload.is_public,
            payload.food_orange,
            payload.food_apple,
            payload.food_banana,
            payload.pub_date,
            payload.qty1,
            payload.qty2,
            payload.qty3,
        ],
    )
    .unwrap();
    
    let id = conn.last_insert_rowid();
    
    let todo = Todo {
        id: Some(id),
        title: payload.title,
        content: payload.content,
        is_public: payload.is_public,
        food_orange: payload.food_orange,
        food_apple: payload.food_apple,
        food_banana: payload.food_banana,
        pub_date: payload.pub_date,
        qty1: payload.qty1,
        qty2: payload.qty2,
        qty3: payload.qty3,
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