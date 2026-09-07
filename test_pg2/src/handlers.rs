use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateTodoRequest, Todo};

/// POST /todos : TODOを新規登録する
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<StatusCode, String> {
    let ret: Vec<Todo> = Vec::new();
    let title = payload.title.trim();
    if title.is_empty() {
        return Ok(StatusCode::BAD_REQUEST);
    }

    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title)
        VALUES ($1)
        RETURNING id, title, completed, created_at
        "#,
    )
    .bind(title)
    .fetch_one(&pool)
    .await.unwrap();

    Ok(StatusCode::CREATED)
}

/// GET /todos : TODO一覧を取得する（登録日時の降順）
pub async fn list_todos(
  State(pool): State<PgPool>
) -> Json<Vec<Todo>> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT id, title, completed, created_at
        FROM todos
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await.unwrap();

    Json(todos)
}

/// DELETE /todos/:id : 指定したIDのTODOを削除する
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> StatusCode {
    let result = sqlx::query(
        r#"
        DELETE FROM todos WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&pool)
    .await.unwrap();

    if result.rows_affected() == 0 {
        return StatusCode::BAD_REQUEST;
    }

    StatusCode::NO_CONTENT
}