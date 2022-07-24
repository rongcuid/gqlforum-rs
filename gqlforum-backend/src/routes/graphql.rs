use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    headers::{Header, HeaderMapExt, HeaderName},
    response::{Html, IntoResponse},
    Extension,
};
use hyper::HeaderMap;
// use sqlx::SqlitePool;

use crate::graphql::{QueryRoot, SchemaRoot};

pub async fn graphql_handler(
    schema: Extension<SchemaRoot>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema
        .execute(req.0)
        .await
        .into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
