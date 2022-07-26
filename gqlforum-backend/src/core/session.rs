use cookie::Cookie;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::{query, Executor, Sqlite, Transaction};

#[derive(Clone, Debug)]
pub struct SessionCookie<'a>(pub Option<Cookie<'a>>);

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: i64,
    secret: String,
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
    tx: &mut Transaction<'_, Sqlite>,
    session_cookie: &SessionCookie<'_>,
) -> Option<SessionData> {
    let cookie = session_cookie.0.as_ref()?;
    let session = SessionData::try_from(cookie).ok()?;
    if verify_session(tx, &session).await {
        Some(session)
    } else {
        None
    }
}

async fn verify_session(tx: &mut Transaction<'_, Sqlite>, session: &SessionData) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(session.secret.as_bytes());
    let token_hash = &hasher.finalize()[..];
    query("SELECT 1 FROM active_sessions WHERE session_user_id = ?1 AND token_hash = ?2 AND DATETIME(expired_at) > DATETIME('now')")
    .bind(session.user_id)
    .bind(token_hash)
    .fetch_optional(tx).await.ok().is_some()
}
