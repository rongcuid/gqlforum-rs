use reqwasm::http::{Request, Response};
use serde::{Deserialize, Serialize};

pub struct Client {
    endpoint: String,
}

#[derive(Serialize, Deserialize)]
struct GraphQLRequest {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variable: Option<String>,
}

impl Client {
    pub fn new(endpoint: &str) -> Self {
        Self {
            endpoint: endpoint.to_owned(),
        }
    }
    pub async fn raw_query(&self, query: &str) -> Result<Response, reqwasm::Error> {
        let request = GraphQLRequest {
            query: Self::minimize_query_string(query),
            variable: None,
        };
        Request::post(&self.endpoint)
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
    }
    fn minimize_query_string(query: &str) -> String {
        let lines: Vec<_> = query.lines().map(|l| l.trim()).collect();
        lines.join(" ")
    }
}
