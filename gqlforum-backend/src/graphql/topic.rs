use async_graphql::*;

use sqlx::{sqlite::SqliteRow, types::time::PrimitiveDateTime, FromRow, Row, SqlitePool};

use crate::core::session::UserCredential;

use super::{post::Post, sql::query_posts_by_topic_id, user::User};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Topic {
    pub id: i64,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub meta: Option<TopicMeta>,
}

#[ComplexObject]
impl Topic {
    #[graphql(complexity = "limit as usize * child_complexity")]
    async fn posts(
        &self,
        ctx: &Context<'_>,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Vec<Post>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        query_posts_by_topic_id(pool, cred.user_id(), self.id, limit, offset)
            .await
            .map_err(Error::from)
    }
}

#[derive(SimpleObject)]
pub struct TopicMeta {
    pub title: String,
    pub author: User,
}

impl<'r> FromRow<'r, SqliteRow> for Topic {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        let title: Option<String> = row.try_get("title")?;
        let user_id: Option<i64> = row.try_get("user_id")?;
        let user_name: Option<String> = row.try_get("username")?;
        let user_signature: Option<String> = row.try_get("post_signature")?;
        Ok(Self {
            id: row.try_get("id")?,
            meta: (|| {
                Some(TopicMeta {
                    author: (User {
                        id: user_id?,
                        name: user_name?,
                        signature: user_signature,
                    }),
                    title: title?,
                })
            })(),
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}
