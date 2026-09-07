use sqlx::MySqlPool;

/// アプリ起動時にテーブルが存在しなければ作成する
pub async fn run_migrations(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
            title TEXT NOT NULL,
            created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
            PRIMARY KEY (id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
