use async_graphql::*;

use sqlx::{query_as, sqlite::SqliteRow, types::time::PrimitiveDateTime, FromRow, Row, SqlitePool};
use tracing::debug;

use crate::core::session::Credential;

use super::{post::Post, user::User};

pub async fn query_topic(
    pool: &SqlitePool,
    user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<Topic>> {
    debug!("Query topic {} for user {:?}", topic_id, user_id);
    let meta = query_topic_meta(pool, user_id, topic_id)
        .await?
        .ok_or(Error::new("Topic does not exist."))?;
    Ok(Some(Topic {
        meta,
        // posts
    }))
}

pub async fn query_topic_meta(
    pool: &SqlitePool,
    _user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<TopicMeta>> {
    let meta = query_as(include_str!("sql/topic_meta.sql"))
        .bind(topic_id)
        .fetch_optional(pool)
        .await?;
    Ok(meta)
}

pub async fn query_topic_posts(
    pool: &SqlitePool,
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
        .fetch_all(pool)
        .await?;
    Ok(posts)
}

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Topic {
    pub meta: TopicMeta,
}

#[ComplexObject]
impl Topic {
    async fn posts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Post>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<Credential>().unwrap();
        query_topic_posts(pool, cred.user_id(), self.meta.id, limit, offset).await
    }
}

#[derive(SimpleObject)]
pub struct TopicMeta {
    pub id: i64,
    pub title: String,
    pub author: User,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

impl<'r> FromRow<'r, SqliteRow> for TopicMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("topic_id")?,
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
