use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    response::{Html, IntoResponse},
    Extension, Json,
};
// use sqlx::SqlitePool;

use crate::graphql::QueryRoot;

pub async fn graphql_handler(
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: Json<Request>,
) -> Json<Response> {
    schema.execute(req.0).await.into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
