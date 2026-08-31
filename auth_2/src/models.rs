use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

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

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct UserRes {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct DbUser {
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Deserialize)]
pub struct UserCreateReq {
    pub email: String,
    pub password: String,
}
