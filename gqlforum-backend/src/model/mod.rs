use async_graphql::*;
use sqlx::{FromRow, SqlitePool};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn user(&self, ctx: &Context<'_>, id: i64) -> Option<User> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE user_id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .unwrap();
        user
    }
    async fn board(&self, ctx: &Context<'_>, id: i64) -> Option<Board> {
        None
    }
    async fn topic(&self, ctx: &Context<'_>, id: i64) -> Option<Topic> {
        None
    }
    async fn post(&self, ctx: &Context<'_>, id: i64) -> Option<Post> {
        None
    }
}

#[derive(SimpleObject)]
pub struct Board {
    id: i64,
}

#[derive(SimpleObject)]
pub struct Topic {
    id: i64,
    author: User,
    board: Board,
}

#[derive(SimpleObject)]
pub struct Post {
    id: i64,
    author: User,
    topic: Topic,
    content: String,
}

#[derive(SimpleObject, FromRow)]
pub struct User {
    id: i64,
    name: String,
}
