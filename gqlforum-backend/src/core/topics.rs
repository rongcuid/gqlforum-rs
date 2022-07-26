use async_graphql::*;

use sqlx::{
    query_as, sqlite::SqliteRow, types::time::PrimitiveDateTime, FromRow, Row, Sqlite, SqlitePool,
    Transaction,
};
use tracing::debug;

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
    pub author: Author,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

impl<'r> FromRow<'r, SqliteRow> for TopicMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            title: row.try_get("title")?,
            author: Author {
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

#[derive(SimpleObject)]
pub struct Post {
    pub meta: PostMeta,
    pub content: Option<PostContent>,
}

impl<'r> FromRow<'r, SqliteRow> for Post {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let meta = PostMeta::from_row(row)?;
        let content = PostContent::from_row(row).ok();
        Ok(Self { meta, content })
    }
}

#[derive(SimpleObject, Debug)]
pub struct PostMeta {
    pub post_number: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

impl<'r> FromRow<'r, SqliteRow> for PostMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            post_number: row.try_get("post_number")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(SimpleObject, Debug)]
pub struct PostContent {
    pub author: Author,
    pub body: String,
}

impl<'r> FromRow<'r, SqliteRow> for PostContent {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let id: Option<i64> = row.try_get("author_user_id")?;
        let name: Option<String> = row.try_get("username")?;
        let signature: Option<String> = row.try_get("post_signature")?;
        let body: Option<String> = row.try_get("body")?;
        let f = ||->Option<Self> {
            Some(Self {
                            author: Author { id: id?, name: name?, signature},
                            body: body?
                        })
        };
        f().ok_or(sqlx::Error::RowNotFound)
    }
}

#[derive(SimpleObject, Debug)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}
impl<'r> FromRow<'r, SqliteRow> for Author {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            signature: row.try_get("signature")?,
        })
    }
}
