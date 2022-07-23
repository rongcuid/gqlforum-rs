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
    pub post_number: i64,
    // pub deleted_at: Option<NaiveDateTime>,
    pub author: Option<Author>,
    pub body: Option<String>,
}
