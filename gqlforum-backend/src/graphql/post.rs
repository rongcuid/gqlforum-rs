use async_graphql::*;
use sqlx::{prelude::*, sqlite::SqliteRow, types::time::PrimitiveDateTime};

use super::user::User;

#[derive(SimpleObject)]
pub struct Post {
    pub meta: PostMeta,
    pub content: Option<PostContent>,
}

impl<'r> FromRow<'r, SqliteRow> for Post {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let meta = PostMeta::from_row(row)?;
        let content = PostContent::from_row(row).ok();
        Ok(Self { meta, content })
    }
}

#[derive(SimpleObject, Debug)]
pub struct PostMeta {
    pub post_number: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

impl<'r> FromRow<'r, SqliteRow> for PostMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            post_number: row.try_get("post_number")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}

#[derive(SimpleObject, Debug)]
pub struct PostContent {
    pub author: User,
    pub body: String,
}

impl<'r> FromRow<'r, SqliteRow> for PostContent {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let id: Option<i64> = row.try_get("author_user_id")?;
        let name: Option<String> = row.try_get("username")?;
        let signature: Option<String> = row.try_get("post_signature")?;
        let body: Option<String> = row.try_get("body")?;
        let f = || -> Option<Self> {
            Some(Self {
                author: User {
                    id: id?,
                    name: name?,
                    signature,
                },
                body: body?,
            })
        };
        f().ok_or(sqlx::Error::RowNotFound)
    }
}
