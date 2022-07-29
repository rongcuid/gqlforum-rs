use sqlx::{query_as, SqlitePool};
use tracing::debug;

use crate::{
    core::session::UserCredential,
    graphql::topic::{Topic, TopicMeta},
};

pub async fn query_topic_by_id(
    pool: &SqlitePool,
    cred: &UserCredential,
    topic_id: i64,
) -> Result<Option<Topic>, sqlx::Error> {
    let user_id = cred.user_id();
    debug!("Query topic {} for user {:?}", topic_id, user_id);
    let meta = query_topic_meta(pool, cred, topic_id).await?;
    let topic = || Some(Topic { meta: meta? });
    Ok(topic())
}

async fn query_topic_meta(
    pool: &SqlitePool,
    _cred: &UserCredential,
    topic_id: i64,
) -> Result<Option<TopicMeta>, sqlx::Error> {
    let meta = query_as(include_str!("topic_meta.sql"))
        .bind(topic_id)
        .fetch_optional(pool)
        .await?;
    Ok(meta)
}
