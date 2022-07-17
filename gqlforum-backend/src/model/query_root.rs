use std::collections::HashMap;

use async_graphql::*;
use sqlx::{query, query_as, sqlite::SqliteRow, FromRow, Row, SqlitePool};

use super::top_down;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn board(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Board> {
        None
    }
    async fn topic(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Topic> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        None
    }
    async fn post(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Post> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        query_file_as!(top_down::Post, "sql/posts.sql", id)
            .fetch_optional(pool)
            .await
            .expect("Query `post` error")
    }
}
