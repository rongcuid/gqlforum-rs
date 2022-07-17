use async_graphql::SimpleObject;


#[derive(SimpleObject)]
pub struct Author {
    pub id: i64,
    pub name: String,
    pub signature: Option<String>,
}

#[derive(SimpleObject)]
pub struct Topic {
    pub author: Author,
    pub title: String,
    pub posts: Vec<Post>,
}

#[derive(SimpleObject)]
pub struct Post {
    pub author: Author,
    pub body: String,
}
