mod db;
mod error;
mod handlers;
mod models;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::MySqlPool,
}

#[tokio::main]
async fn main() {
    // .env 読み込み
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL must be set (.env を確認してください)");
    let server_addr =
        std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // MySQL コネクションプール作成
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("MySQL への接続に失敗しました");

    // マイグレーション実行（テーブルが無ければ作成）
    db::run_migrations(&pool)
        .await
        .expect("マイグレーションの実行に失敗しました");

    let state = AppState { pool };

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/todos", post(handlers::create_todo).get(handlers::list_todos))
        .route("/todos/:id", axum::routing::delete(handlers::delete_todo))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr: SocketAddr = server_addr.parse().expect("SERVER_ADDR の形式が不正です");
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
