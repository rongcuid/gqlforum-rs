use serde::{Deserialize, Serialize};
use serde_json::json;
use sycamore::{prelude::*, suspense::Suspense};

use crate::graphql::GraphQLClient;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Topic {
    id: i64,
    created_at: String,
    updated_at: Option<String>,
    deleted_at: Option<String>,
    meta: Option<TopicMeta>,
    posts: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TopicMeta {
    title: String,
    author: Author,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Author {
    name: String,
    signature: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    post_number: i64,
    meta: PostMeta,
    created_at: String,
    updated_at: Option<String>,
    deleted_at: Option<String>,
    content: Option<PostContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostMeta {
    author: Author,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostContent {
    body: String,
}

#[component]
async fn TopicContent<G: Html>(cx: Scope<'_>, props: (i64, i64)) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let resp = client
        .query_raw_with(
            r#"query($id: Int, $offset: Int) {
                topic(topicId: $id) {
                    id
                    createdAt
                    updatedAt
                    deletedAt
                    meta {
                      title
                      author {
                        name
                        signature
                      }
                    }
                    posts(limit: 10, offset: $offset) {
                      createdAt
                      updatedAt
                      deletedAt
                      postNumber
                      meta {
                        author {
                          name
                          signature
                        }
                      }
                      content {
                        body
                      }
                    }
                  }
            }"#,
            json!({
                "id": props.0,
                "offset": (props.1 - 1) * 10
            }),
        )
        .await
        .unwrap();
    if let Some(data) = resp.data {
        let topic: Topic = serde_json::from_value(data.get("topic").unwrap().clone()).unwrap();
        view! { cx, (format!("{:?}", topic)) }
    } else {
        view! { cx, "Topic does not exist!"}
    }
}

#[component]
pub fn Topic<G: Html>(cx: Scope<'_>, props: (i64, i64)) -> View<G> {
    view! { cx,
        Suspense {
            fallback: view! { cx, "Loading..." },
            TopicContent(props)
        }
    }
}
