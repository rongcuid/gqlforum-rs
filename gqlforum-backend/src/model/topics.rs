use async_graphql::SimpleObject;


#[derive(SimpleObject)]
pub struct Author {
    pub id: i64,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: Option<DateTime<Utc>>,
    // pub last_seen: Option<DateTime<Utc>>,
    // #[sqlx(rename = "username")]
    pub name: String,
    pub signature: Option<String>,
}

// #[derive(SimpleObject, FromRow)]
// pub struct Board {
//     pub id: i64,
//     pub topics: Vec<Topic>,
// }

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
