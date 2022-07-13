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
        query_as::<_, top_down::Post>("SELECT * FROM posts, users WHERE posts.author_user_id = users.user_id AND post_id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .expect("Query `post` error")
    }
}
