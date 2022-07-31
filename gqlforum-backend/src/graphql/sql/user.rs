use sqlx::sqlite::SqliteRow;
use sqlx::{query, query_as, Row, SqliteExecutor};

use crate::core::session::UserCredential;
use crate::graphql::{
    session::Role,
    user::{User, UserBy},
};

pub async fn query_user<'e, E: SqliteExecutor<'e>>(
    pool: E,
    _cred: &UserCredential,
    by: UserBy,
) -> Result<Option<User>, sqlx::Error> {
    let user = match by {
        UserBy::Id(id) => {
            query_as("SELECT id, username, post_signature FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(pool)
                .await?
        }
        UserBy::Name(username) => {
            query_as("SELECT id, username, post_signature FROM users WHERE username = ?")
                .bind(username)
                .fetch_optional(pool)
                .await?
        }
    };
    Ok(user)
}

pub async fn query_role<'e, E: SqliteExecutor<'e>>(
    _pool: E,
    user_id: i64,
) -> Result<Option<Role>, sqlx::Error> {
    if user_id == 1 {
        return Ok(Some(Role::Administrator));
    }
    Ok(Some(Role::Regular))
}

pub async fn query_user_topic_ids<'e, E: SqliteExecutor<'e>>(
    executor: E,
    cred: &UserCredential,
    user_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<i64>, sqlx::Error> {
    query(include_str!("user_topics.sql"))
        .bind(cred.user_id())
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .map(|row: SqliteRow| row.get("id"))
        .fetch_all(executor)
        .await
}

pub async fn query_user_post_ids<'e, E: SqliteExecutor<'e>>(
    executor: E,
    cred: &UserCredential,
    user_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<i64>, sqlx::Error> {
    query(include_str!("user_posts.sql"))
        .bind(cred.user_id())
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .map(|row: SqliteRow| row.get("id"))
        .fetch_all(executor)
        .await
}
