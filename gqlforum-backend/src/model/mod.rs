pub mod leaf;
pub mod top_down;

use async_graphql::*;
use sqlx::{query, query_as, sqlite::SqliteRow, FromRow, Row, SqlitePool};

use self::leaf::User;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn board(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Board> {
        None
    }
    async fn topic(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Topic> {
        None
    }
    async fn post(&self, ctx: &Context<'_>, id: i64) -> Option<top_down::Post> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        query("SELECT * FROM posts, users WHERE posts.author_user_id = users.user_id AND post_id = ?")
            .bind(id)
            .map(|row: SqliteRow| {
                top_down::Post { 
                    id: row.get("post_id"), 
                    author: User { 
                        id: row.get("user_id"), 
                        name: row.get("user_name") 
                    }, 
                    content: row.get("content") 
                }
            })
            .fetch_optional(pool)
            .await
            .expect("Query `post` error")
    }
}
