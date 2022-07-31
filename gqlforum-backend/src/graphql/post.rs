use async_graphql::*;
use sqlx::{prelude::*, sqlite::SqliteRow, types::time::PrimitiveDateTime};

use super::user::User;

#[derive(SimpleObject, Clone)]
pub struct Post {
    pub post_number: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub meta: Option<PostMeta>,
    pub content: Option<PostContent>,
}

impl<'r> FromRow<'r, SqliteRow> for Post {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let meta = PostMeta::from_row(row).ok();
        let content = PostContent::from_row(row).ok();
        Ok(Self {
            meta,
            content,
            post_number: row.try_get("post_number")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(SimpleObject, Debug, Clone)]
pub struct PostMeta {
    pub author: User,
}

impl<'r> FromRow<'r, SqliteRow> for PostMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            author: User {
                id: row.try_get("author_user_id")?,
                name: row.try_get("username")?,
                signature: row.try_get("post_signature")?,
            },
        })
    }
}

#[derive(SimpleObject, Debug, Clone)]
pub struct PostContent {
    pub body: String,
}

impl<'r> FromRow<'r, SqliteRow> for PostContent {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let body: Option<String> = row.try_get("body")?;
        let f = || -> Option<Self> { Some(Self { body: body? }) };
        f().ok_or(sqlx::Error::RowNotFound)
    }
}
