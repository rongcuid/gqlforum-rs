use async_graphql::*;

use sqlx::{sqlite::SqliteRow, types::time::PrimitiveDateTime, FromRow, Row, SqlitePool};

use crate::core::{session::Credential, sql::query_posts_by_topic_id};

use super::{post::Post, user::User};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Topic {
    pub meta: TopicMeta,
}

#[ComplexObject]
impl Topic {
    async fn posts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Post>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<Credential>().unwrap();
        query_posts_by_topic_id(pool, cred.user_id(), self.meta.id, limit, offset)
            .await
            .map_err(Error::from)
    }
}

#[derive(SimpleObject)]
pub struct TopicMeta {
    pub id: i64,
    pub title: String,
    pub author: User,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
}

impl<'r> FromRow<'r, SqliteRow> for TopicMeta {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("topic_id")?,
            title: row.try_get("title")?,
            author: User {
                id: row.try_get("user_id")?,
                name: row.try_get("username")?,
                signature: row.try_get("post_signature")?,
            },
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}
