use async_graphql::SimpleObject;
use sqlx::{
    types::chrono::{DateTime, Utc},
    FromRow,
};

#[derive(SimpleObject, FromRow)]
pub struct User {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
    #[sqlx(rename = "username")]
    pub name: String,
    pub signature: Option<String>,
}
