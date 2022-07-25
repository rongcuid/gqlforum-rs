use async_graphql::*;

use sqlx::{query_file, types::time::PrimitiveDateTime, Sqlite, Transaction};
use tracing::debug;

pub async fn query_topic_meta(
    tx: &mut Transaction<'_, Sqlite>,
    _user_id: Option<i64>,
    topic_id: i64,
) -> Result<Option<TopicMeta>> {
    let meta = query_file!("sql/topic_meta.sql", topic_id)
        .map(|row| TopicMeta {
            title: row.title,
            author: Author {
                id: row.user_id,
                name: row.username,
                signature: row.post_signature,
            },
        })
        .fetch_optional(tx)
        .await?;
    Ok(meta)
}

pub async fn query_topic_posts(
    tx: &mut Transaction<'_, Sqlite>,
    user_id: Option<i64>,
    topic_id: i64,
) -> Result<Vec<Post>> {
    debug!("Querying for posts");
    let posts = query_file!("sql/topic_by_id.sql", user_id, topic_id)
        .map(|row| {
            let f = || {
                let author = Author {
                    id: row.author_user_id?,
                    name: row.username?,
                    signature: row.post_signature,
                };
                let body = row.body?;
                Some(PostContent {
                    author,
                    body,
                    created_at: row.created_at?,
                    updated_at: row.updated_at,
                })
            };
            let content = f();
            Post {
                post_number: row.post_number,
                deleted_at: row.deleted_at,
                content,
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
}

#[derive(SimpleObject)]
pub struct Post {
    pub post_number: i64,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub content: Option<PostContent>,
}

#[derive(SimpleObject)]
pub struct PostContent {
    pub author: Author,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub body: String,
}

#[derive(SimpleObject)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}
