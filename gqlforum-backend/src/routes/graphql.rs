use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};
use hyper::HeaderMap;
// use sqlx::SqlitePool;

use crate::graphql::QueryRoot;

pub struct AuthData {
    pub user_id: i64,
}

pub async fn graphql_handler(
    _headers: HeaderMap,
    schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let _user_id = 1; // TODO!
    schema
        .execute(req.0.data(AuthData { user_id: 1 }))
        .await
        .into()
}

pub async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
