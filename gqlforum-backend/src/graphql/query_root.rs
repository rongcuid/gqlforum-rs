use async_graphql::*;
use sqlx::{query, query_file, types::chrono::DateTime, Row, SqlitePool};
use tracing::{info, warn, error};

pub struct QueryRoot;

use crate::model::topics;

#[Object]
impl QueryRoot {
    // async fn board(&self, _ctx: &Context<'_>, _id: i64) -> Option<top_down::Board> {
    //     None
    // }
    async fn topic(&self, ctx: &Context<'_>, user_id: i64, topic_id: i64) -> Option<topics::Topic> {
        info!("Topic tracing");
        let pool = ctx.data::<SqlitePool>().unwrap();
        let mut tx = pool
            .begin()
            .await
            .expect("Failed to begin transaction for `topic`");
        // TODO Placeholder
        let meta = query_file!("sql/topic_meta.sql", topic_id)
            .fetch_optional(&mut tx)
            .await
            .expect("Failed on `topic` select topic metadata")?;
        let posts = query_file!("sql/topic_by_id.sql", user_id, topic_id)
            .map(|row| {
                let f = || -> Option<topics::Author> {
                    Some(topics::Author {
                        id: row.author_user_id?,
                        name: row.username?,
                        signature: row.post_signature,
                    })
                };
                let author: Option<topics::Author> = f();
                topics::Post {
                    post_number: row.post_number,
                    deleted_at: row.deleted_at,
                    author,
                    body: row.body,
                }
            })
            .fetch_all(&mut tx)
            .await
            .expect("Failed on `topic` select post");
        tx.commit().await.expect("Failed on `topic` commit");
        Some(topics::Topic {
            author: topics::Author {
                id: meta.user_id,
                name: meta.username,
                signature: meta.post_signature,
            },
            title: meta.title,
            posts,
        })
    }
}
