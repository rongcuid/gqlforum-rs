use async_graphql::*;

use hmac::Mac;

use sqlx::{Row, SqlitePool};

pub struct QueryRoot;

use crate::{
    core::{
        authentication::{try_get_verified_session_data, SessionCookie},
        topics::{self, query_topic},
    },
    startup::HmacSecret,
};

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
        let _key = ctx.data::<HmacSecret>().unwrap();
        let session_cookie = ctx.data::<SessionCookie>().unwrap();
        let mut tx = pool.begin().await?;
        let session_data = try_get_verified_session_data(&mut tx, session_cookie).await;
        tx.commit().await?;

        query_topic(
            pool,
            session_data.map(|d| d.user_id),
            topic_id,
            limit,
            offset,
            ctx.look_ahead().field("posts").exists(),
        )
        .await
    }
}
