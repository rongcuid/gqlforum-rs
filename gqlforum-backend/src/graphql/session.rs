use async_graphql::futures_util::TryStreamExt;
use itertools::Itertools;
use sqlx::sqlite::SqliteRow;
use std::sync::Arc;
use std::{collections::HashMap, marker::PhantomData};

use async_graphql::dataloader::*;
use async_graphql::*;
use sqlx::{query_as, SqliteExecutor, SqlitePool, FromRow, Row};

use super::user::User;

#[derive(SimpleObject)]
pub struct Session {
    pub user: User,
    pub role: Role,
}

#[derive(Enum, Clone, Copy, PartialEq, Eq, Debug)]
pub enum Role {
    Administrator,
    Moderator,
    Regular,
}

impl<'r> FromRow<'r, SqliteRow> for Role {
    fn from_row(row: &'r SqliteRow) -> Result<Self, sqlx::Error> {
        Ok(match row.try_get("role")? {
            "ADMINISTRATOR" => Self::Administrator,
            "MODERATOR" => Self::Moderator,
            _ => Self::Regular,
        })
    }
}
