use std::collections::HashMap;

use async_graphql::*;
use sqlx::{query, query_as, sqlite::SqliteRow, FromRow, Row, SqlitePool};

pub struct QueryRoot;

use crate::model::topics;

#[Object]
impl QueryRoot {
    // async fn board(&self, _ctx: &Context<'_>, _id: i64) -> Option<top_down::Board> {
    //     None
    // }
    // async fn topic(
    //     &self,
    //     ctx: &Context<'_>,
    //     user_id: i64,
    //     topic_id: i64,
    // ) -> Option<Topic> {
    //     let pool = ctx.data::<SqlitePool>().unwrap();
    //     // let mut tx = pool.begin().await.expect("Failed to begin transaction for `topic`");
    //     // query_as::<_, topics::Topic>(include_str!("../../sql/topic_by_id.sql"))
    //     //     .bind(user_id)
    //     //     .bind(topic_id)
    //     //     .fetch_all(pool)
    //     //     .await
    //     //     .expect("Query `topic` error")
    //     todo!()
    // }
    async fn post(&self, ctx: &Context<'_>, id: i64) -> Option<topics::Post> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        query_as!(topics::Post, "SELECT body FROM posts WHERE id = ?", id)
            .fetch_optional(pool)
            .await
            .expect("Query `post` error")
    }
}
