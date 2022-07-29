use async_graphql::*;

use super::user::User;

#[derive(SimpleObject)]
pub struct Session {
    pub user: User,
    pub role: Role,
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    Administrator,
    Moderator,
    Regular,
}
