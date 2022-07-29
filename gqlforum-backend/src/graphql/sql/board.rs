use sqlx::{query, sqlite::SqliteRow, Executor, Row, SqliteExecutor};

use crate::core::session::UserCredential;

pub async fn query_board_topic_ids<'e, E: SqliteExecutor<'e>>(
    executor: E,
    cred: &UserCredential,
    limit: i64,
    offset: i64,
) -> Result<Vec<i64>, sqlx::Error> {
    let user_id = cred.user_id();
    query(include_str!("board_topics.sql"))
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .map(|row: SqliteRow| row.get("id"))
        .fetch_all(executor)
        .await
}
