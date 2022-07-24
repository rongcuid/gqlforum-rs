use async_graphql::*;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(&self, user: String, password: String) -> Option<i64> {
        Some(1)
    }
}