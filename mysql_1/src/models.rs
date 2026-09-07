use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// DB から取得する Todo モデル
#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub created_at: NaiveDateTime,
}

/// 登録リクエストのボディ
#[derive(Debug, Deserialize)]
pub struct CreateTodoRequest {
    pub title: String,
}

/// 汎用エラーレスポンス
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}
