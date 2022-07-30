use async_graphql::*;

use sqlx::SqlitePool;

pub struct QueryRoot;

use crate::backend::core::session::UserCredential;

use super::{
    session::Session,
    sql::{query_board_topic_ids, query_topic_by_id, query_user},
    topic,
    user::{User, UserBy},
};

#[Object]
impl QueryRoot {
    async fn session(&self, ctx: &Context<'_>) -> Result<Option<Session>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        if let Some(id) = cred.user_id() {
            let user = query_user(pool, cred, UserBy::Id(id)).await?;
            let f = || Some(Session { user: user? });
            Ok(f())
        } else {
            Ok(None)
        }
    }
    async fn user(&self, ctx: &Context<'_>, by: UserBy) -> Result<Option<User>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        query_user(pool, cred, by).await.map_err(Error::from)
    }
    async fn board(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<topic::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        let topic_ids = query_board_topic_ids(pool, cred, limit, offset).await?;
        // N+1 query here
        let mut v = Vec::new();
        for id in topic_ids {
            if let Some(topic) = query_topic_by_id(pool, cred, id).await? {
                v.push(topic);
            }
        }
        Ok(v)
    }

    async fn topic(&self, ctx: &Context<'_>, topic_id: i64) -> Result<Option<topic::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();

        query_topic_by_id(pool, cred, topic_id)
            .await
            .map_err(Error::from)
    }
}
