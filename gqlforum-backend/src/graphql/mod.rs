pub mod mutation_root;
pub mod query_root;
pub mod subscription_root;

pub mod post;
mod sql;
pub mod topic;
pub mod user;

use async_graphql::{EmptySubscription, Schema};
pub use mutation_root::MutationRoot;
pub use query_root::QueryRoot;
pub use subscription_root::SubscriptionRoot;

pub type SchemaRoot = Schema<QueryRoot, MutationRoot, EmptySubscription>;
