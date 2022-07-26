use async_graphql::*;

use sqlx::{
    query, query_file, sqlite::SqliteRow, types::time::PrimitiveDateTime, Row, Sqlite, Transaction,
};
use tracing::debug;

pub async fn query_topic_meta(
    tx: &mut Transaction<'_, Sqlite>,
    _user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<TopicMeta>> {
    let meta = query(include_str!("sql/topic_meta.sql"))
        .bind(topic_id)
        .map(|row: SqliteRow| TopicMeta {
            title: row.get("title"),
            author: Author {
                id: row.get("user_id"),
                name: row.get("username"),
                signature: row.get("post_signature"),
            },
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            deleted_at: row.get("deleted_at"),
        })
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
    debug!("Querying for posts");
    let posts = query(include_str!("sql/topic_by_id.sql"))
        .bind(user_id)
        .bind(topic_id)
        .bind(limit)
        .bind(offset)
        .map(|row: SqliteRow| {
            let f = || {
                let author = Author {
                    id: row.try_get("author_user_id").ok()?,
                    name: row.try_get("username").ok()?,
                    signature: row.try_get("post_signature").ok(),
                };
                let body = row.try_get("body").ok()?;
                Some(PostContent { author, body })
            };
            Post {
                meta: PostMeta {
                    post_number: row.get("post_number"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    deleted_at: row.get("deleted_at"),
                },
                content: f(),
            }
        })
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

#[derive(SimpleObject)]
pub struct Post {
    pub meta: PostMeta,
    pub content: Option<PostContent>,
}

#[derive(SimpleObject)]
pub struct PostMeta {
    pub post_number: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(SimpleObject)]
pub struct PostContent {
    pub author: Author,
    pub body: String,
}

#[derive(SimpleObject)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}
