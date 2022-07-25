use async_graphql::*;

use sqlx::{Row, SqlitePool};
use tracing::debug;

pub struct QueryRoot;

use crate::graphql::topics::{query_topic_meta, query_topic_posts};

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
        let meta = query_topic_meta(&mut tx, user_id, topic_id)
            .await?
            .ok_or(Error::new("Topic does not exist."))?;
        let posts = if ctx.look_ahead().field("posts").exists() {
            debug!("Querying for posts");
            query_topic_posts(&mut tx, user_id, topic_id).await?
        } else {
            Vec::new()
        };
        tx.commit().await?;
        Ok(Some(topics::Topic { meta, posts }))
    }
}
