use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    error::AppError,
    models::{CreateTodoRequest, Todo},
    AppState,
};

/// ヘルスチェック用
pub async fn health() -> &'static str {
    "OK"
}

/// TODO 登録
/// POST /todos  { "title": "買い物に行く" }
pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), AppError> {
    let title = payload.title.trim();
    if title.is_empty() {
        return Err(AppError::BadRequest("title は必須です".to_string()));
    }

    let result = sqlx::query("INSERT INTO todos (title) VALUES (?)")
        .bind(title)
        .execute(&state.pool)
        .await?;

    let id = result.last_insert_id();

    let todoOne = Todo {
        id,
        title: title.to_string(),
        created_at: chrono::Local::now().naive_local(),
    };
    /*
    let todo = sqlx::query_as::<_, Todo>("SELECT id, title, created_at FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(&state.pool)
        .await?;
    */

    Ok((StatusCode::CREATED, Json(todoOne)))
}

/// TODO 一覧取得
/// GET /todos
pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT id, title, created_at FROM todos ORDER BY id DESC",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(todos))
}

/// TODO 削除
/// DELETE /todos/:id
pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM todos WHERE id = ?")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("id={} の TODO は見つかりません", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
