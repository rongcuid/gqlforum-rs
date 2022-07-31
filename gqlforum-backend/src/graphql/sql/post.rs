use sqlx::{query, query_as, sqlite::SqliteRow, Row, Sqlite, SqliteExecutor, Transaction};

use crate::{core::session::UserCredential, graphql::post::Post};

pub async fn query_posts_by_topic_id<'e, E: SqliteExecutor<'e>>(
    pool: E,
    user_id: Option<i64>,
    topic_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>, sqlx::Error> {
    let posts = query_as(include_str!("posts_by_topic_id.sql"))
        .bind(user_id)
        .bind(topic_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
    Ok(posts)
}

pub async fn query_post_by_id<'e, E: SqliteExecutor<'e>>(
    pool: E,
    cred: &UserCredential,
    post_id: i64,
) -> Result<Option<Post>, sqlx::Error> {
    let topic = query_as(include_str!("post_by_id.sql"))
        .bind(cred.user_id())
        .bind(post_id)
        .fetch_optional(pool)
        .await?;
    Ok(topic)
}

pub async fn new_post(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: i64,
    topic_id: i64,
    body: String,
) -> Result<i64, sqlx::Error> {
    query(
        r"INSERT INTO posts (topic_id, author_user_id, body)
        VALUES (?3, ?1, ?2) RETURNING id",
    )
    .bind(user_id)
    .bind(body)
    .bind(topic_id)
    .map(|row: SqliteRow| row.get("id"))
    .fetch_one(&mut *tx)
    .await
}
