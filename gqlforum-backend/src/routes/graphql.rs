use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use axum_extra::extract::CookieJar;
use sqlx::SqlitePool;

// use sqlx::SqlitePool;

use crate::core::{
    cookies::verify_cookie_unchecked,
    session::{try_get_verified_session_data, SessionCookie, UserCredential},
};
use crate::graphql::SchemaRoot;
use crate::startup::{HmacSecret, SessionCookieName};

fn get_session_cookie<'a>(
    jar: &CookieJar,
    name: &SessionCookieName,
    key: &HmacSecret,
) -> SessionCookie<'a> {
    let cookie = jar
        .get(&name.0)
        .cloned()
        .and_then(|x| verify_cookie_unchecked(x, key.0.as_bytes()));
    SessionCookie(cookie)
}

pub async fn graphql_handler(
    jar: CookieJar,
    Extension(pool): Extension<SqlitePool>,
    Extension(name): Extension<SessionCookieName>,
    Extension(key): Extension<HmacSecret>,
    schema: Extension<SchemaRoot>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let cookie = get_session_cookie(&jar, &name, &key);
    let session_data = try_get_verified_session_data(&pool, &cookie).await;
    schema
        .execute(req.0.data(UserCredential::new(session_data)))
        .await
        .into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
