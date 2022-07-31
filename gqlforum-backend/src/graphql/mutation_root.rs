use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use async_graphql::*;
use cookie::{time::OffsetDateTime, Cookie};
use nanoid::nanoid;
use secrecy::Secret;
use sqlx::{query, sqlite::SqliteRow, Row, SqlitePool};

use crate::core::{
    authentication::{change_password, register, validate_user_credentials},
    cookies::sign_cookie_unchecked,
    session::{delete_session, insert_session, invalidate_session, SessionData, UserCredential},
};
use crate::startup::{HmacSecret, SessionCookieName};

use super::{
    post::Post,
    sql::{
        delete_post, new_post, new_topic, query_post_by_id, query_post_permission,
        query_topic_by_id, query_user,
    },
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

        invalidate_session(ctx, pool, &cred).await?;
        Ok(true)
    }
    async fn register(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<User> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();

        let mut tx = pool.begin().await?;
        let result = register(&mut tx, cred, username, password).await?;
        tx.commit().await?;
        Ok(result)
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
    async fn new_topic(&self, ctx: &Context<'_>, title: String, body: String) -> Result<Topic> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        if let Some(user_id) = cred.user_id() {
            let mut tx = pool.begin().await?;
            let topic_id = new_topic(&mut tx, user_id, title, body).await?;
            let topic = query_topic_by_id(&mut tx, cred, topic_id).await?.unwrap();
            tx.commit().await?;
            Ok(topic)
        } else {
            Err(Error::new("Must be logged in to post."))
        }
    }

    async fn new_post(&self, ctx: &Context<'_>, topic_id: i64, body: String) -> Result<Post> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        if let Some(user_id) = cred.user_id() {
            let mut tx = pool.begin().await?;
            let post_id = new_post(&mut tx, user_id, topic_id, body).await?;
            let post = query_post_by_id(&mut tx, cred, post_id).await?.unwrap();
            tx.commit().await?;
            Ok(post)
        } else {
            Err(Error::new("Must be logged in to post."))
        }
    }

    async fn delete_post(&self, ctx: &Context<'_>, post_id: i64) -> Result<i64> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let cred = ctx.data::<UserCredential>().unwrap();
        if let Some(user_id) = cred.user_id() {
            let mut tx = pool.begin().await?;
            let permission = query_post_permission(&mut tx, Some(user_id), post_id).await?;
            if !permission.can_write() {
                return Err(Error::new("Permission denied."));
            }
            let post_id = delete_post(&mut tx, post_id).await?;
            tx.commit().await?;
            Ok(post_id)
        } else {
            Err(Error::new("Must be logged in to delete post."))
        }
    }
}
