use async_graphql::*;

use sqlx::{Row, SqlitePool};
use tracing::debug;

pub struct QueryRoot;

use crate::graphql::topics::query_topic;

use super::topics;

#[Object]
impl QueryRoot {
    async fn topics(
        &self,
        _ctx: &Context<'_>,
        _topic_id: i64,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Result<Vec<topics::Topic>> {
        todo!()
    }

    async fn topic(
        &self,
        ctx: &Context<'_>,
        topic_id: i64,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Option<topics::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let user_id = None; // TODO
        debug!("Querying for topics");

        query_topic(
            pool,
            user_id,
            topic_id,
            limit,
            offset,
            ctx.look_ahead().field("posts").exists(),
        )
        .await
    }
}
