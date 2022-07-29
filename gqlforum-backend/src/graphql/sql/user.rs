use sqlx::{query_as, SqlitePool};

use crate::{
    core::session::UserCredential,
    graphql::user::{User, UserBy},
};

pub async fn query_user(
    pool: &SqlitePool,
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
