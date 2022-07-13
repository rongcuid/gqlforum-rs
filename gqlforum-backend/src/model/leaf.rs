use async_graphql::SimpleObject;
use sqlx::FromRow;

#[derive(SimpleObject, FromRow)]
pub struct User {
    pub id: i64,
    pub name: String,
}
