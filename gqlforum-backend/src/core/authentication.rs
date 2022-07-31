use argon2::{Argon2, PasswordHash, PasswordVerifier};
use secrecy::{ExposeSecret, Secret};
use sqlx::{query_as, sqlite::SqliteRow, FromRow, Row, SqliteExecutor};
use tracing::instrument;

use crate::telemetry::spawn_blocking_with_tracing;

struct UserCredentials {
    id: i64,
    phc_string: Secret<String>,
}

impl<'r> FromRow<'r, SqliteRow> for UserCredentials {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            id: row.try_get("id")?,
            phc_string: Secret::new(row.try_get("phc_string")?),
        })
    }
}

pub async fn validate_user_credentials<'e, E: SqliteExecutor<'e>>(
    pool: E,
    username: String,
    password: Secret<String>,
) -> Option<i64> {
    let cred = fetch_user_credentials(pool, username).await.ok()??;
    spawn_blocking_with_tracing(move || verify_password_hash(cred, password))
        .await
        .ok()?
}

async fn fetch_user_credentials<'e, E: SqliteExecutor<'e>>(
    pool: E,
    username: String,
) -> Result<Option<UserCredentials>, sqlx::Error> {
    let cred = query_as("SELECT id, phc_string FROM users WHERE username = ?1")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(cred)
}

#[instrument(name = "Verify password hash", skip(credential, password))]
fn verify_password_hash(credential: UserCredentials, password: Secret<String>) -> Option<i64> {
    let hash = PasswordHash::new(credential.phc_string.expose_secret()).ok()?;
    Argon2::default()
        .verify_password(password.expose_secret().as_bytes(), &hash)
        .ok()?;
    Some(credential.id)
}
