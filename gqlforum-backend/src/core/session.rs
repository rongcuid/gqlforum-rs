use cookie::Cookie;
use secrecy::{ExposeSecret, Secret};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{query, Sqlite, Transaction, SqlitePool};
use tracing::debug;

#[derive(Clone, Debug)]
pub struct SessionCookie<'a>(pub Option<Cookie<'a>>);

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
    debug!("CHK SESSION: {:?}", session.secret);
    if verify_session(pool, &session).await {
        Some(session)
    } else {
        None
    }
}

async fn verify_session(pool: &SqlitePool, session: &SessionData) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(session.secret.as_bytes());
    let token_hash = &hasher.finalize()[..];
    query("SELECT 1 FROM active_sessions WHERE session_user_id = ?1 AND token_hash = ?2 AND DATETIME(expired_at) > DATETIME('now')")
    .bind(session.user_id)
    .bind(token_hash)
    .fetch_optional(pool).await.ok().is_some()
}

pub async fn insert_session(
    pool: &SqlitePool,
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

pub async fn delete_session(
    pool: &SqlitePool,
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
