use async_graphql::SimpleObject;
use sqlx::{sqlite::SqliteRow, FromRow, Row};

use super::leaf::User;

// #[derive(SimpleObject, FromRow)]
// pub struct Board {
//     pub id: i64,
//     pub topics: Vec<Topic>,
// }

#[derive(SimpleObject)]
pub struct Topic {
    pub author: User,
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(SimpleObject)]
pub struct Post {
    // pub author: User,
    pub body: String,
}

// impl<'r> FromRow<'r, SqliteRow> for Post {
//     fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
//         Ok(Self {
//             author: User::from_row(row)?,
//             content: row.try_get("content")?,
//         })
//     }
// }
