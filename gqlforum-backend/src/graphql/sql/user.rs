use sqlx::{query, query_as, Executor, Sqlite, SqliteExecutor};

use crate::{
    core::session::UserCredential,
    graphql::{
        session::Role,
        user::{User, UserBy},
    },
};

pub async fn query_user<'e, E: SqliteExecutor<'e>>(
    pool: E,
    _cred: &UserCredential,
    by: UserBy,
) -> Result<Option<User>, sqlx::Error> {
    let user = match by {
        UserBy::Id(id) => {
            query_as("SELECT id, username, post_signature FROM users WHERE id = ?")
                .bind(id)
                .fetch_optional(pool)
                .await?
        }
        UserBy::Name(username) => {
            query_as("SELECT id, username, post_signature FROM users WHERE username = ?")
                .bind(username)
                .fetch_optional(pool)
                .await?
        }
    };
    Ok(user)
}

pub async fn query_role<'e, E: SqliteExecutor<'e>>(
    pool: E,
    user_id: i64,
) -> Result<Option<Role>, sqlx::Error> {
    if user_id == 1 {
        return Ok(Some(Role::Administrator));
    }
    let is_moderator = query("SELECT 1 FROM moderators WHERE moderator_user_id = ?")
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .is_some();
    if is_moderator {
        Ok(Some(Role::Moderator))
    } else {
        Ok(Some(Role::Regular))
    }
}
