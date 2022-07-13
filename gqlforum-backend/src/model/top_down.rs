use async_graphql::SimpleObject;
use sqlx::{FromRow, Row, sqlite::SqliteRow};

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

#[derive(SimpleObject)]
pub struct Post {
    pub id: i64,
    pub author: User,
    pub content: String,
}

impl<'r> FromRow<'r, SqliteRow> for Post {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self { 
            id: row.try_get("post_id")?, 
            author: User::from_row(row)?, 
            content: row.try_get("content")?
        })
    }
}
