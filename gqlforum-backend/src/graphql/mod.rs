pub mod query_root;
pub mod topics;
pub mod mutation_root;

use async_graphql::{EmptySubscription, EmptyMutation, Schema};
pub use query_root::QueryRoot;
pub use mutation_root::MutationRoot;

pub type SchemaRoot = Schema<QueryRoot, EmptyMutation, EmptySubscription>;