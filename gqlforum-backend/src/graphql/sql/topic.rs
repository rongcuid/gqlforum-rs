use sqlx::{query_as, Executor, Sqlite};
use tracing::debug;

use crate::{
    core::session::UserCredential,
    graphql::topic::{Topic, TopicMeta},
};

pub async fn query_topic_by_id<'e, E: Executor<'e, Database = Sqlite>>(
    pool: E,
    cred: &UserCredential,
    topic_id: i64,
) -> Result<Option<Topic>, sqlx::Error> {
    let user_id = cred.user_id();
    debug!("Query topic {} for user {:?}", topic_id, user_id);
    let meta = query_topic_meta(pool, cred, topic_id).await?;
    let topic = || Some(Topic { meta: meta? });
    Ok(topic())
}

async fn query_topic_meta<'e, E: Executor<'e, Database = Sqlite>>(
    pool: E,
    _cred: &UserCredential,
    topic_id: i64,
) -> Result<Option<TopicMeta>, sqlx::Error> {
    let meta = query_as(include_str!("topic_meta.sql"))
        .bind(topic_id)
        .fetch_optional(pool)
        .await?;
    Ok(meta)
}
