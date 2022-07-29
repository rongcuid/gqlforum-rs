use sqlx::{query_as, SqlitePool};
use tracing::debug;

use crate::graphql::{
    post::Post,
    topic::{Topic, TopicMeta},
};

pub async fn query_topic(
    pool: &SqlitePool,
    user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<Topic>, sqlx::Error> {
    debug!("Query topic {} for user {:?}", topic_id, user_id);
    let meta = query_topic_meta(pool, user_id, topic_id).await?;
    let topic = || Some(Topic { meta: meta? });
    Ok(topic())
}

pub async fn query_topic_meta(
    pool: &SqlitePool,
    _user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<TopicMeta>, sqlx::Error> {
    let meta = query_as(include_str!("topic_meta.sql"))
        .bind(topic_id)
        .fetch_optional(pool)
        .await?;
    Ok(meta)
}

pub async fn query_posts_by_topic_id(
    pool: &SqlitePool,
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
