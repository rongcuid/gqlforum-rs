use async_graphql::*;

use hmac::Mac;

use sqlx::{Row, SqlitePool};

pub struct QueryRoot;

use crate::core::session::Credential;

use super::{
    topic::{self, query_topic},
    user::{User, UserBy},
};

#[Object]
impl QueryRoot {
    async fn user(&self, _by: UserBy) -> Result<User> {
        Err(Error::new("unimplemented"))
    }
    async fn topics(
        &self,
        _ctx: &Context<'_>,
        _topic_id: i64,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Result<Vec<topic::Topic>> {
        Err(Error::new("unimplemented"))
    }

    async fn topic(&self, ctx: &Context<'_>, topic_id: i64) -> Result<Option<topic::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let session_data = ctx.data::<Credential>().unwrap();

        query_topic(pool, session_data.0.as_ref().map(|d| d.user_id), topic_id).await
    }
}
