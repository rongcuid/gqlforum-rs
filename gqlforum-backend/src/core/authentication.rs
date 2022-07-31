use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use async_graphql::Error;
use secrecy::{ExposeSecret, Secret};
use sqlx::{query, query_as, sqlite::SqliteRow, FromRow, Row, Sqlite, SqliteExecutor, Transaction};
use tracing::instrument;

use crate::{
    graphql::{
        sql::query_user,
        user::{User, UserBy},
    },
    telemetry::spawn_blocking_with_tracing,
};

use super::session::{delete_session, UserCredential};

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

pub fn hash_password(password: Secret<String>) -> Result<String, async_graphql::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.expose_secret().as_bytes(), &salt)?
        .to_string())
}

pub async fn change_password(
    tx: &mut Transaction<'_, Sqlite>,
    cred: &UserCredential,
    current_password: String,
    new_password: String,
) -> Result<User, async_graphql::Error> {
    if let Some(session) = cred.session() {
        let username = query("SELECT username FROM users WHERE id = ?")
            .bind(session.user_id)
            .map(|row: SqliteRow| row.get("username"))
            .fetch_one(&mut *tx)
            .await?;
        let user_id =
            validate_user_credentials(&mut *tx, username, Secret::new(current_password)).await;
        if user_id != cred.user_id() {
            return Err(Error::new("user id does not match session!"));
        }
        // Delete the current session
        delete_session(
            &mut *tx,
            session.user_id,
            Secret::new(session.secret.clone()),
        )
        .await?;
        let phc: String =
            spawn_blocking_with_tracing(|| hash_password(Secret::new(new_password))).await??;
        query("UPDATE users SET phc_string = ?2 WHERE id = ?1")
            .bind(user_id)
            .bind(phc)
            .execute(&mut *tx)
            .await?;
        Ok(query_user(&mut *tx, cred, UserBy::Id(user_id.unwrap()))
            .await?
            .unwrap())
    } else {
        Err(Error::new("You must log in first"))
    }
}

pub async fn register(
    tx: &mut Transaction<'_, Sqlite>,
    cred: &UserCredential,
    username: String,
    password: String,
) -> Result<User, async_graphql::Error> {
    if cred.user_id().is_some() {
        return Err(Error::new("Cannot register while logged in."));
    }
    let phc: String =
        spawn_blocking_with_tracing(|| hash_password(Secret::new(password))).await??;
    query("INSERT OR ROLLBACK INTO users(username, phc_string) VALUES (?, ?)")
        .bind(&username)
        .bind(phc)
        .execute(&mut *tx)
        .await?;
    Ok(query_user(&mut *tx, cred, UserBy::Name(username))
        .await?
        .unwrap())
}
