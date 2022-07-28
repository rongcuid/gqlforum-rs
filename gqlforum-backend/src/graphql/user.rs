use async_graphql::*;
use sqlx::{prelude::*, sqlite::SqliteRow};

use super::{topic::Topic, post::Post};

#[derive(SimpleObject, Debug)]
#[graphql(complex)]
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

#[ComplexObject]
impl User {
    async fn topics(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Vec<Topic> {
        todo!()
    }

    async fn posts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Vec<Post> {
        todo!()
    }
}
