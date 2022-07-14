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
        let result = query("
        SELECT 
            topics.*, posts.*, users.user_id as topic_user_id, users.user_name as topic_user_name
        FROM
            topics 
            INNER JOIN users on topics.author_user_id = users.user_id
            INNER JOIN (
                SELECT 
                    posts.author_user_id p_a_u_id, posts.topic_id, users.user_name as p_u_n
                FROM 
                    posts INNER JOIN users ON posts.post_id = users.user_id
            ) on topics.topic_id = posts.topic_id
        WHERE topics.topic_id = ?
        GROUP BY posts.post_id
        ")
        .bind(id)
        .fetch_all(pool)
        .map(|rows: Vec<SqliteRow>| {
            
        })
        .await
        .expect("Query `topic` error");
        None
    }
    async fn post(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Post> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        query_as::<_, top_down::Post>("SELECT posts.*, users.user_id, users.user_name FROM posts, users WHERE posts.author_user_id = users.user_id AND post_id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .expect("Query `post` error")
    }
}
