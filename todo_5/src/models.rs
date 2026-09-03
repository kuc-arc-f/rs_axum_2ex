use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DBから取得するTodoの構造体（todosテーブルの全カラムに対応）
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub content: Option<String>,
    pub is_public: Option<bool>,
    pub food_orange: Option<bool>,
    pub food_apple: Option<bool>,
    pub food_banana: Option<bool>,
    pub pub_date: Option<String>,
    pub qty1: Option<i32>,
    pub qty2: Option<i32>,
    pub qty3: Option<i32>,
    pub created_at: DateTime<Utc>,
}

/// TODO登録時のリクエストボディ
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
    pub content: Option<String>,
    pub is_public: Option<bool>,
    pub food_orange: Option<bool>,
    pub food_apple: Option<bool>,
    pub food_banana: Option<bool>,
    pub pub_date: Option<String>,
    pub qty1: Option<i32>,
    pub qty2: Option<i32>,
    pub qty3: Option<i32>,
}

/// エラーレスポンス用
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}
