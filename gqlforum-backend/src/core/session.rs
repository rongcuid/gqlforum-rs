use async_graphql::Context;
use cookie::{time::OffsetDateTime, Cookie};
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{query, SqliteExecutor, SqlitePool};
use tracing::debug;

use crate::startup::SessionCookieName;

#[derive(Clone, Debug)]
pub struct SessionCookie<'a>(pub Option<Cookie<'a>>);

pub struct UserCredential(Option<SessionData>);

impl UserCredential {
    pub fn new(session: Option<SessionData>) -> Self {
        Self(session)
    }
    pub fn is_anonymous(&self) -> bool {
        self.0.is_none()
    }
    pub fn user_id(&self) -> Option<i64> {
        Some(self.0.as_ref()?.user_id)
    }
    pub fn session(&self) -> Option<&SessionData> {
        self.0.as_ref()
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: i64,
    pub secret: String,
}

impl<'a> TryFrom<&Cookie<'a>> for SessionData {
    type Error = String;

    fn try_from(cookie: &Cookie) -> Result<Self, Self::Error> {
        let val = cookie.value();
        serde_json::from_str(val).map_err(|_| "invalid session cookie".to_owned())
    }
}

/// Try to get verified session data from a session cookie
pub async fn try_get_verified_session_data<'a>(
    pool: &SqlitePool,
    session_cookie: &SessionCookie<'_>,
) -> Option<SessionData> {
    let cookie = session_cookie.0.as_ref()?;
    let session = SessionData::try_from(cookie).ok()?;
    if verify_session(pool, &session).await {
        debug!("Valid session for user_id: {}", session.user_id);
        Some(session)
    } else {
        debug!("Invalid session for user_id: {}", session.user_id);
        None
    }
}

async fn verify_session<'e, E: SqliteExecutor<'e>>(pool: E, session: &SessionData) -> bool {
    let token_hash = &Sha256::digest(session.secret.as_bytes())[..];
    query(
        r#"
        SELECT 1 
        FROM active_sessions 
        WHERE session_user_id = ?1 
            AND token_hash = ?2
            AND datetime('now') < expires_at"#,
    )
    .bind(session.user_id)
    .bind(token_hash)
    .fetch_optional(pool)
    .await
    .unwrap()
    .is_some()
}

pub async fn insert_session<'e, E: SqliteExecutor<'e>>(
    pool: E,
    session: SessionData,
) -> Result<(), sqlx::Error> {
    debug!("NEW SESSION: {:?}", session.secret);
    let hash = &Sha256::digest(session.secret.as_bytes())[..];
    query(
        r#"INSERT INTO 
        active_sessions(session_user_id, token_hash, expires_at)
        VALUES (?1, ?2, DATETIME(CURRENT_TIMESTAMP, '+30 days'))
        "#,
    )
    .bind(session.user_id)
    .bind(hash)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_session<'e, E: SqliteExecutor<'e>>(
    pool: E,
    user_id: i64,
    secret: Secret<String>,
) -> Result<(), sqlx::Error> {
    let hash = &Sha256::digest(secret.expose_secret().as_bytes())[..];
    query(
        r#"DELETE FROM active_sessions
            WHERE session_user_id = ?1 AND token_hash = ?2"#,
    )
    .bind(user_id)
    .bind(hash)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn invalidate_session<'e, E: SqliteExecutor<'e>>(
    ctx: &Context<'_>,
    pool: E,
    cred: &UserCredential,
) -> Result<(), async_graphql::Error> {
    if let Some(session) = cred.session() {
        let session_cookie_name = ctx.data::<SessionCookieName>().unwrap();
        // Remove the cookie
        let cookie = Cookie::build(session_cookie_name.0.clone(), "")
            .http_only(true)
            .secure(true)
            .same_site(cookie::SameSite::Strict)
            .expires(OffsetDateTime::now_utc())
            .finish();
        ctx.append_http_header("Set-Cookie", cookie.to_string());
        delete_session(pool, session.user_id, Secret::new(session.secret.clone())).await?;
        Ok(())
    } else {
        Err(async_graphql::Error::new("Already logged out"))
    }
}
