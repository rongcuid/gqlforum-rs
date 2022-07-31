use async_graphql::*;
use sqlx::{prelude::*, sqlite::SqliteRow, SqlitePool};

use crate::core::session::UserCredential;

use super::{
    session::Role,
    sql::{query_role, query_topic_by_id, query_user_topic_ids},
    topic::Topic,
};

#[derive(Debug, OneofObject)]
pub enum UserBy {
    Name(String),
    Id(i64),
}

#[derive(SimpleObject, Debug, Clone)]
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
            name: row.try_get("username")?,
            signature: row.try_get("post_signature")?,
        })
    }
}

#[ComplexObject]
impl User {
    async fn role(&self, ctx: &Context<'_>) -> Result<Role> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        query_role(pool, self.id)
            .await?
            .ok_or(Error::new("user does not exist"))
    }
    #[graphql(complexity = "limit as usize * child_complexity")]
    async fn topics(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        let topic_ids = query_user_topic_ids(pool, cred, self.id, limit, offset).await?;
        // N+1 query here
        let mut v = Vec::new();
        for id in topic_ids {
            if let Some(topic) = query_topic_by_id(pool, cred, id).await? {
                v.push(topic);
            }
        }
        Ok(v)
    }
}
