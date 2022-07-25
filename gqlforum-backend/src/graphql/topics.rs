use async_graphql::SimpleObject;
use sqlx::types::time::PrimitiveDateTime;

#[derive(SimpleObject)]
pub struct Topic {
    pub author: Author,
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(SimpleObject)]
pub struct Post {
    pub post_number: i64,
    pub deleted_at: Option<PrimitiveDateTime>,
    pub content: Option<PostContent>,
}

#[derive(SimpleObject)]
pub struct PostContent {
    pub author: Author,
    pub created_at: PrimitiveDateTime,
    pub updated_at: Option<PrimitiveDateTime>,
    pub body: String,
}

#[derive(SimpleObject)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}
