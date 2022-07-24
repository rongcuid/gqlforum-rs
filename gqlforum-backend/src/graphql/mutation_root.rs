use async_graphql::*;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<String> {
        Ok("Stub implementation".to_owned())
    }
}
