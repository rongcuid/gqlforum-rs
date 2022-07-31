use serde::{Deserialize, Serialize};
use serde_json::json;
use sycamore::{prelude::*, suspense::Suspense};

use crate::graphql::GraphQLClient;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Topic {
    id: i64,
    created_at: String,
    updated_at: Option<String>,
    deleted_at: Option<String>,
    meta: Option<TopicMeta>,
    posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TopicMeta {
    title: String,
    author: Author,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Author {
    name: String,
    signature: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Post {
    post_number: i64,
    meta: Option<PostMeta>,
    created_at: String,
    updated_at: Option<String>,
    deleted_at: Option<String>,
    content: Option<PostContent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostMeta {
    author: Author,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    let topic: &Signal<Option<Topic>> = create_signal(cx, None);
    let posts = create_memo(cx, || {
        (*topic.get()).clone().map(|x| x.posts).unwrap_or_default()
    });
    if let Some(data) = resp.data {
        topic.set(serde_json::from_value(data.get("topic").unwrap().clone()).unwrap());
        let topic = (*topic.get()).clone().unwrap();
        if topic.meta.is_none() {
            return view! {cx, "Deleted" };
        }
        let meta = topic.meta.unwrap();
        view! { cx,
            h1 { (meta.title) }
            p { "-- by " em {(meta.author.name)} }
            Indexed {
                iterable: posts,
                view: |cx, post| view! { cx,
                    div {
                        h2 { (post.post_number) }
                        p { ((||{
                            let body = post.content.as_ref()?.body.clone();
                            Some(body)
                        })().unwrap_or("[DELETED]".to_owned()))
                        }
                        p {
                            "-- by " em {(
                                (||{
                                    let meta = post.meta.as_ref()?.clone();
                                    Some(meta.author.name)
                                })().unwrap_or("[REDACTED]".to_owned())
                            )}
                        }
                    }
                }
            }
        }
    } else {
        view! { cx, "Topic does not exist!"}
    }
}

struct NewPostInput {
    topic_id: i64,
    body: String,
}

#[component]
async fn NewPostOutput<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let input = use_context::<ReadSignal<NewPostInput>>(cx);
    let resp = client
        .query_raw_with(
            r#"
        mutation($topicId: String, $body: String) {
            newPost(topicId: $topicId, body: $body) {
                id
            }
        }
        "#,
            json!({
                "topicId": input.get().topic_id,
                "body": input.get().body,
            }),
        )
        .await
        .unwrap();
    let errors = create_signal(cx, Vec::new());
    view! {cx, (
        if let Some(errs) = &resp.errors {
            errors.set(errs.iter().map(|x| x.message.clone()).collect());
            view! { cx,
                ul {
                    Indexed {
                        iterable: errors,
                        view: |cx, x| view! { cx,
                            li { (x) }
                        }
                    }
                }
            }
        } else if let Some(_data) = &resp.data {
            view! { cx, p {"Topic posted"}}
        } else {
            view! {cx, p {"Internal Server Error"}}
        }
    )}
}

struct TopicId(i64);

#[component]
fn NewPost<G: Html>(cx: Scope<'_>) -> View<G> {
    let submitted = create_signal(cx, false);
    let topic_id = use_context::<TopicId>(cx);
    let body = create_signal(cx, String::new());
    provide_context_ref(
        cx,
        create_memo(cx, || NewPostInput {
            topic_id: topic_id.0,
            body: (*body.get()).clone(),
        }),
    );
    let new_topic = |_| {
        submitted.set(true);
    };

    view! {
        cx,
        form {
            div {
                textarea(type="text", bind:value=body)
            }
            div {
                button(on:click=new_topic,type="button") { "New Post" }
            }
            div {
                (if *submitted.get() { view! { cx,
                        Suspense {
                            fallback: view! {cx, "Posting..."},
                            NewPostOutput { }
                        }
                    } } else { view! {cx, } })
                }
        }
    }
}

#[component]
pub fn Topic<G: Html>(cx: Scope<'_>, props: (i64, i64)) -> View<G> {
    provide_context(cx, TopicId(props.0));
    view! { cx,
        Suspense {
            fallback: view! { cx, "Loading..." },
            TopicContent(props)
        }
        NewPost {}
    }
}
