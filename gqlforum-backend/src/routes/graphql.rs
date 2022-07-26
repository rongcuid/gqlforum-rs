use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use axum_extra::extract::SignedCookieJar;
// use sqlx::SqlitePool;

use crate::{core::authentication::SessionCookie, graphql::SchemaRoot, startup::SessionCookieName};

pub async fn graphql_handler(
    jar: SignedCookieJar,
    Extension(name): Extension<SessionCookieName>,
    schema: Extension<SchemaRoot>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let cookie = SessionCookie(jar.get(&name.0));
    schema.execute(req.0.data(cookie)).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
