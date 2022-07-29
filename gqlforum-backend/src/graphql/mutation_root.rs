use async_graphql::*;
use cookie::Cookie;
use nanoid::nanoid;
use secrecy::Secret;
use sqlx::SqlitePool;

use crate::{
    core::{
        authentication::validate_user_credentials,
        cookies::sign_cookie_unchecked,
        session::{delete_session, insert_session, SessionData, UserCredential},
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
        let cred = ctx.data::<UserCredential>().unwrap();

        if cred.user_id().is_some() {
            Err(Error::new("Already logged in"))
        } else if let Some(user_id) =
            validate_user_credentials(pool, username, Secret::new(password)).await
        {
            let session = SessionData {
                user_id,
                secret: nanoid!(),
            };
            let cookie = Cookie::build(
                session_cookie_name.0.clone(),
                serde_json::to_string(&session)?,
            )
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .finish();
            let cookie = sign_cookie_unchecked(cookie, key.0.as_bytes());
            ctx.append_http_header("Set-Cookie", cookie.to_string());
            insert_session(pool, session).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();

        if let Some(session) = cred.session() {
            delete_session(pool, session.user_id, Secret::new(session.secret.clone())).await?;
            Ok(true)
        } else {
            Err(Error::new("Already logged out"))
        }
    }
}
