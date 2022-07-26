use std::{sync::Arc, pin::Pin};

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
// use sqlx::SqlitePool;

use crate::graphql::SchemaRoot;

pub async fn graphql_handler(
    schema: Extension<SchemaRoot>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.0).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
