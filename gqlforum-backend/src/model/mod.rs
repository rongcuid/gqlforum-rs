use async_graphql::*;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn board(&self) -> Option<Board> {
        None
    }
}

#[derive(SimpleObject)]
pub struct Board {
    id: u64,
    topics: Vec<Topic>,
}

#[derive(SimpleObject)]
pub struct Topic {
    id: u64,
    author: User,
}

#[derive(SimpleObject)]
pub struct Post {
    id: u64,
    author: User,
    content: String,
}

#[derive(SimpleObject)]
pub struct Reply {
    id: u64,
    author: User,
}

#[derive(SimpleObject)]
pub struct User {
    id: u64,
    name: String,
}
