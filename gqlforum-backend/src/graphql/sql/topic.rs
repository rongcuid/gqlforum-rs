use sqlx::{query_as, SqliteExecutor};

use crate::core::session::UserCredential;
use crate::graphql::topic::Topic;

pub async fn query_topic_by_id<'e, E: SqliteExecutor<'e>>(
    pool: E,
    cred: &UserCredential,
    topic_id: i64,
) -> Result<Option<Topic>, sqlx::Error> {
    let topic = query_as(include_str!("topic_by_id.sql"))
        .bind(cred.user_id())
        .bind(topic_id)
        .fetch_optional(pool)
        .await?;
    Ok(topic)
}
