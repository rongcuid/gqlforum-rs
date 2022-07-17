use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(SimpleObject, FromRow)]
pub struct User {
    #[sqlx(rename = "user_id")]
    pub id: i64,
    #[sqlx(rename = "user_name")]
    pub name: String,
}
