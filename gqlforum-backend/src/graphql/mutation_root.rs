use async_graphql::*;
use cookie::Cookie;
use nanoid::nanoid;

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<String> {
        let cookie = Cookie::build("test_cookie", nanoid!())
            .secure(true)
            .http_only(true)
            .finish();
        ctx.append_http_header("Set-Cookie", cookie.to_string());
        Ok("Stub implementation".to_owned())
    }
}
