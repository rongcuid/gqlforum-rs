use axum::{response::Html, Extension};
use sqlx::{SqlitePool};

pub async fn handler(Extension(pool): Extension<SqlitePool>) -> Html<String> {
    let x = sqlx::query_scalar("SELECT '<h1>Hello, world from SQLite3.'")
        .fetch_one(&pool)
        .await
        .expect("Query error");
    Html(x)
}