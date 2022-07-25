use async_graphql::*;

use sqlx::{query_file, Row, SqlitePool};
use tracing::debug;

pub struct QueryRoot;

use crate::graphql::topics::query_topic_posts;

use super::topics;

#[Object]
impl QueryRoot {
    // async fn board(&self, _ctx: &Context<'_>, _id: i64) -> Option<top_down::Board> {
    //     None
    // }
    async fn topic(&self, ctx: &Context<'_>, topic_id: i64) -> Result<Option<topics::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let user_id = Some(1); // TODO
        debug!("Querying for topics");
        let mut tx = pool.begin().await?;
        let meta = query_file!("sql/topic_meta.sql", topic_id)
            .fetch_optional(&mut tx)
            .await?
            .ok_or(Error::new("Internal Server Error"))?;
        let posts = if ctx.look_ahead().field("posts").exists() {
            debug!("Querying for posts");
            query_topic_posts(&mut tx, user_id, topic_id).await?
        } else {
            Vec::new()
        };
        Ok(Some(topics::Topic {
            author: topics::Author {
                id: meta.user_id,
                name: meta.username,
                signature: meta.post_signature,
            },
            title: meta.title,
            posts,
        }))
    }
}
