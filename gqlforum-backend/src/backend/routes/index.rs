use axum::{response::Html, Extension};
use sqlx::SqlitePool;

pub async fn index_handler(Extension(pool): Extension<SqlitePool>) -> Html<String> {
    let version: String = sqlx::query_scalar("SELECT sqlite_version();")
        .fetch_one(&pool)
        .await
        .expect("Query error");
    Html(format!(
        "<h1>Hello world GQLForum</h1>
        <ul>
        <li>SQLite: {}</li>
        <li>GraphQL: async-graphql <a href=\"/graphql\">/graphql</a></li>
        </ul>",
        version
    ))
}
