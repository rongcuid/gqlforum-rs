use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_graphql::*;
use cookie::{Cookie, time::OffsetDateTime};
use nanoid::nanoid;
use secrecy::Secret;
use sqlx::{query, sqlite::SqliteRow, Row, SqlitePool};

use crate::core::{
    authentication::{validate_user_credentials, change_password},
    cookies::sign_cookie_unchecked,
    session::{delete_session, insert_session, SessionData, UserCredential},
};
use crate::startup::{HmacSecret, SessionCookieName};

use super::{
    post::Post,
    sql::query_user,
    topic::Topic,
    user::{User, UserBy},
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
        let session_cookie_name = ctx.data::<SessionCookieName>().unwrap();

        if let Some(session) = cred.session() {
            let cookie = Cookie::build(session_cookie_name.0.clone(), "")
                .http_only(true)
                .secure(true)
                .same_site(cookie::SameSite::Strict)
                .expires(OffsetDateTime::now_utc())
                .finish();
            ctx.append_http_header("Set-Cookie", cookie.to_string());
            delete_session(pool, session.user_id, Secret::new(session.secret.clone())).await?;
            Ok(true)
        } else {
            Err(Error::new("Already logged out"))
        }
    }
    async fn register(
        &self,
        _ctx: &Context<'_>,
        _username: String,
        _password: String,
    ) -> Result<User> {
        Err(Error::new("Unimplemented"))
    }
    async fn change_password(
        &self,
        ctx: &Context<'_>,
        current_password: String,
        new_password: String,
    ) -> Result<User> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        let mut tx = pool.begin().await?;
        let result = change_password(&mut tx, cred, current_password, new_password).await?;
        tx.commit().await?;
        Ok(result)
    }
    async fn new_topic(&self, _ctx: &Context<'_>, _title: String, _body: String) -> Result<Topic> {
        Err(Error::new("Unimplemented"))
    }
    async fn edit_topic(&self, _ctx: &Context<'_>, _id: i64, _title: String) -> Result<Topic> {
        Err(Error::new("Unimplemented"))
    }
    async fn new_post(&self, _ctx: &Context<'_>, _topic_id: i64, _body: String) -> Result<Post> {
        Err(Error::new("Unimplemented"))
    }
    async fn edit_post(&self, _ctx: &Context<'_>, _id: i64, _body: String) -> Result<Post> {
        Err(Error::new("Unimplemented"))
    }
}
