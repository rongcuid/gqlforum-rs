use async_graphql::*;
use cookie::Cookie;
use nanoid::nanoid;
use secrecy::Secret;
use sqlx::SqlitePool;

use crate::{
    core::{
        authentication::validate_user_credentials,
        session::{try_get_verified_session_data, SessionCookie}, cookies::sign_cookie_unchecked,
    },
    startup::{HmacSecret, SessionCookieName},
};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn login(&self, ctx: &Context<'_>, username: String, password: String) -> Result<bool> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let key = ctx.data::<HmacSecret>().unwrap();
        let session_cookie_name = ctx.data::<SessionCookieName>().unwrap();
        let session_cookie = ctx.data::<SessionCookie>().unwrap();

        let mut tx = pool.begin().await?;
        let session = try_get_verified_session_data(&mut tx, session_cookie).await;
        tx.commit().await?;

        if let Some(_) = session {
            Err(Error::new("Already logged in"))
        } else {
            if let Some(user_id) =
                validate_user_credentials(pool, username, Secret::new(password)).await
            {
                let cookie = Cookie::build(session_cookie_name.0.clone(), nanoid!())
                    .http_only(true)
                    .secure(true)
                    .same_site(cookie::SameSite::Strict)
                    .finish();
                let cookie = sign_cookie_unchecked(cookie, key.0.as_bytes());
                ctx.append_http_header("Set-Cookie", cookie.to_string());
                // TODO Not inserted yet
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}
