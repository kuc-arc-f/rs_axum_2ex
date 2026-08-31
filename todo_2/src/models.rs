use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// DBから取得するTodoの構造体
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

/// TODO登録時のリクエストボディ
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

/// エラーレスポンス用
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}