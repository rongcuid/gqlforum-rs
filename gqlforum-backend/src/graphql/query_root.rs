use async_graphql::*;

use sqlx::SqlitePool;

pub struct QueryRoot;

use crate::core::session::UserCredential;

use super::{
    sql::{query_topic_by_id, query_user},
    topic,
    user::{User, UserBy},
};

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        query_user(pool, cred, by).await.map_err(Error::from)
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
        let cred = ctx.data::<UserCredential>().unwrap();

        query_topic_by_id(pool, cred, topic_id)
            .await
            .map_err(Error::from)
    }
}
