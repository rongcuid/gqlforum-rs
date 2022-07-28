use async_graphql::*;

use sqlx::{
    query_as, sqlite::SqliteRow, types::time::PrimitiveDateTime, FromRow, Row, Sqlite, SqlitePool,
    Transaction,
};
use tracing::debug;

use super::{post::Post, user::User};

pub async fn query_topic(
    pool: &SqlitePool,
    user_id: Option<i64>,
    topic_id: i64,
    limit: i64,
    offset: i64,
    query_posts: bool,
) -> Result<Option<Topic>> {
    debug!("Query topic {} for user {:?}", topic_id, user_id);
    let mut tx = pool.begin().await?;
    let meta = query_topic_meta(&mut tx, user_id, topic_id)
        .await?
        .ok_or(Error::new("Topic does not exist."))?;
    let posts = if query_posts {
        query_topic_posts(&mut tx, user_id, topic_id, limit, offset).await?
    } else {
        Vec::new()
    };
    tx.commit().await?;
    Ok(Some(Topic { meta, posts }))
}

pub async fn query_topic_meta(
    tx: &mut Transaction<'_, Sqlite>,
    _user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<TopicMeta>> {
    let meta = query_as(include_str!("sql/topic_meta.sql"))
        .bind(topic_id)
        .fetch_optional(tx)
        .await?;
    Ok(meta)
}

pub async fn query_topic_posts(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: Option<i64>,
    topic_id: i64,
    limit: i64,
    offset: i64,
) -> Result<Vec<Post>> {
    let posts = query_as(include_str!("sql/topic_by_id.sql"))
        .bind(user_id)
        .bind(topic_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(tx)
        .await?;
    Ok(posts)
}

#[derive(SimpleObject)]
pub struct Topic {
    pub meta: TopicMeta,
    pub posts: Vec<Post>,
}

#[derive(SimpleObject)]
pub struct TopicMeta {
    pub title: String,
    pub author: User,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

impl<'r> FromRow<'r, SqliteRow> for TopicMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            title: row.try_get("title")?,
            author: User {
                id: row.try_get("user_id")?,
                name: row.try_get("username")?,
                signature: row.try_get("post_signature")?,
            },
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}
