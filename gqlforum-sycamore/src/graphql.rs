use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct GraphQLClient {
    endpoint: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphQLRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLRawResponse {
    pub data: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GraphQLError {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<GraphQLSourceMap>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<GraphQLPathNode>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GraphQLSourceMap {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GraphQLPathNode {
    Field(String),
    Index(usize),
}

impl GraphQLClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
        }
    }

    /// Lowest level query, returning the http response directly.
    pub async fn query_http(&self, query: &str) -> Result<Response, reqwasm::Error> {
        let request = GraphQLRequest {
            query: Self::minimize_query_string(query),
            variable: None,
        };
        Request::post(&self.endpoint)
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
    }

    /// Raw query returning JSON structure
    pub async fn query_raw(&self, query: &str) -> Result<GraphQLRawResponse, reqwasm::Error> {
        let response = self
            .query_http(query)
            .await?
            .json::<GraphQLRawResponse>()
            .await
            .unwrap();
        Ok(response)
    }
    fn minimize_query_string(query: &str) -> String {
        let lines: Vec<_> = query.lines().map(|l| l.trim()).collect();
        lines.join(" ")
    }
}
