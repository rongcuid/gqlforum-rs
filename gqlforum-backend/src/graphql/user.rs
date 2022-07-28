use async_graphql::*;
use sqlx::{prelude::*, sqlite::SqliteRow};

#[derive(SimpleObject, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}
impl<'r> FromRow<'r, SqliteRow> for User {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            name: row.try_get("name")?,
            signature: row.try_get("signature")?,
        })
    }
}
