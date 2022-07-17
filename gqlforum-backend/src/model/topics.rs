use async_graphql::SimpleObject;
use sqlx::types::chrono::{DateTime, Utc, NaiveDateTime};


#[derive(SimpleObject)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}

#[derive(SimpleObject)]
pub struct Topic {
    pub author: Author,
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(SimpleObject)]
pub struct Post {
    pub deleted_at: Option<NaiveDateTime>,
    pub author: Option<Author>,
    pub body: Option<String>,
}
