use std::sync::Arc;

use async_graphql::*;

use cookie::Cookie;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use sqlx::{Row, SqlitePool};
use tracing::{debug, trace};

pub struct QueryRoot;

use crate::{
    core::{
        cookies::{sign_cookie_unchecked, verify_cookie_unchecked},
        topics::{self, query_topic},
    },
    startup::HmacSecret,
};

#[Object]
impl QueryRoot {
    async fn topics(
        &self,
        _ctx: &Context<'_>,
        _topic_id: i64,
        #[graphql(default = 10)] _limit: i64,
        #[graphql(default = 0)] _offset: i64,
    ) -> Result<Vec<topics::Topic>> {
        todo!()
    }

    async fn topic(
        &self,
        ctx: &Context<'_>,
        topic_id: i64,
        #[graphql(default = 10)] limit: i64,
        #[graphql(default = 0)] offset: i64,
    ) -> Result<Option<topics::Topic>> {
        let pool = ctx.data::<SqlitePool>().unwrap();
        let key = ctx.data::<HmacSecret>().unwrap();
        let user_id = None; // TODO
        debug!("Querying for topics");

        // From cookie crate
        let cookie = Cookie::build("test-cookie", "HELLO")
            .same_site(cookie::SameSite::Strict)
            .http_only(true)
            .secure(true)
            .finish();
        let cookie = sign_cookie_unchecked(cookie, key.0.as_bytes());

        ctx.append_http_header("Set-Cookie", cookie.to_string());

        query_topic(
            pool,
            user_id,
            topic_id,
            limit,
            offset,
            ctx.look_ahead().field("posts").exists(),
        )
        .await
    }
}
