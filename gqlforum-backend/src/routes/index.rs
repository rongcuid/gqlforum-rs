use axum::{response::Html, Extension};
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};

pub async fn index_handler(Extension(db): Extension<DatabaseConnection>) -> Html<String> {
    let version: String = db
        .query_one(Statement::from_sql_and_values(
            DbBackend::Sqlite,
            "SELECT sqlite_version()",
            [],
        ))
        .await
        .expect("SELECT sqlite_version() error")
        .expect("sqlite_version() no result")
        .try_get("", "sqlite_version()")
        .unwrap();
    Html(format!(
        "<h1>Hello world GQLForum</h1>
        <ul>
        <li>SQLite: {}</li>
        <li>GraphQL: async-graphql <a href=\"/graphql\">/graphql</a></li>
        </ul>",
        version
    ))
}
