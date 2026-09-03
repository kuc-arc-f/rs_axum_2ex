use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{CreateTodoRequest, ErrorResponse, Todo};

/// POST /todos : TODOを新規登録する
/// 成功時: 201 CREATED + 登録されたTodoをJSONで返却
/// 失敗時: 400 / 500 + ErrorResponse
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateTodoRequest>,
) -> Result<(StatusCode, Json<Todo>), (StatusCode, Json<ErrorResponse>)> {
    // バリデーション: title は必須（空文字は不可）
    let title = payload.title.trim().to_string();
    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: "title is required".to_string(),
            }),
        ));
    }

    // todosテーブルの全項目を登録（未指定の項目はNULL）
    let todo = sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (
            title, content, is_public,
            food_orange, food_apple, food_banana,
            pub_date, qty1, qty2, qty3
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING
            id, title, content, is_public,
            food_orange, food_apple, food_banana,
            pub_date, qty1, qty2, qty3, created_at
        "#,
    )
    .bind(title)
    .bind(payload.content)
    .bind(payload.is_public)
    .bind(payload.food_orange)
    .bind(payload.food_apple)
    .bind(payload.food_banana)
    .bind(payload.pub_date)
    .bind(payload.qty1)
    .bind(payload.qty2)
    .bind(payload.qty3)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        tracing::error!("TODO登録に失敗しました: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "failed to create todo".to_string(),
            }),
        )
    })?;

    Ok((StatusCode::CREATED, Json(todo)))
}

/// GET /todos : TODO一覧を取得する（登録日時の降順）
pub async fn list_todos(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Todo>>, (StatusCode, Json<ErrorResponse>)> {
    let todos = sqlx::query_as::<_, Todo>(
        r#"
        SELECT
            id, title, content, is_public,
            food_orange, food_apple, food_banana,
            pub_date, qty1, qty2, qty3, created_at
        FROM todos
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("TODO一覧の取得に失敗しました: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "failed to list todos".to_string(),
            }),
        )
    })?;

    Ok(Json(todos))
}

/// DELETE /todos/:id : 指定したIDのTODOを削除する
pub async fn delete_todo(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResponse>)> {
    let result = sqlx::query(r#"
        DELETE FROM todos WHERE id = $1
        "#)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("TODO削除に失敗しました: {e}");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                error: "failed to delete todo".to_string(),
            }),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorResponse {
                error: "todo not found".to_string(),
            }),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}