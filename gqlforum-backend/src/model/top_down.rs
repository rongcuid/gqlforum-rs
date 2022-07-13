use async_graphql::SimpleObject;
use sqlx::FromRow;

use super::leaf::User;

#[derive(SimpleObject, FromRow)]
pub struct Board {
    pub id: i64,
    pub topics: Vec<Topic>,
}

#[derive(SimpleObject, FromRow)]
pub struct Topic {
    pub id: i64,
    pub author: User,
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(SimpleObject, FromRow)]
pub struct Post {
    pub id: i64,
    pub author: User,
    pub content: String,
}

