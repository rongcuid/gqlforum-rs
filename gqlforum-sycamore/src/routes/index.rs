use serde_json::Value;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

use crate::components::NewTopic;
use crate::graphql::GraphQLClient;

#[component]
fn IndexTopics<G: Html>(cx: Scope<'_>) -> View<G> {
    let topics = use_context::<Vec<Value>>(cx);
    let views = View::new_fragment(
        topics
            .iter()
            .map(|x| {
                let id = x.get("id").unwrap().as_i64().unwrap();
                let title = x
                    .get("meta")
                    .unwrap()
                    .get("title")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned();
                let author = x
                    .get("meta")
                    .unwrap()
                    .get("author")
                    .unwrap()
                    .get("name")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned();
                let createdAt = x.get("createdAt").unwrap().as_str().unwrap().to_owned();
                view! {cx, li {
                   strong { a(href=format!("/topic/{}/1", id)) { (title) } }
                   " --by "
                   em { (author) }
                   " on "
                   (createdAt)
                } }
            })
            .collect(),
    );

    view! {cx,
        ul {
            (views)
        }
    }
}

#[component]
async fn IndexPage<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let resp = client
        .query_raw(
            r#"
    query {
        session {
            user {
                id
                name
                role
            }
        }
        boardTopics {
            id
            meta {
                title
                author {
                    name
                }
            }
            createdAt
            updatedAt
            deletedAt
        }
    }
    "#,
        )
        .await
        .unwrap();
    let left = view! { cx,
            span {a(href="/") { "Home" } }
    };
    let mut logged_in = false;
    if let Some(data) = resp.data {
        let right = if let Some(user) = data.get("session").unwrap().get("user") {
            logged_in = true;
            let id = user.get("id").unwrap().as_i64().unwrap();
            let name = user.get("name").unwrap().as_str().unwrap().to_owned();
            let role = user.get("role").unwrap().as_str().unwrap().to_owned();
            view! { cx,
                span {
                    a(href=format!("/user/{}", id)) { (format!("{} ({})", name, role)) }
                    " | "
                    a(href="/logout") { "Logout" }
                }
            }
        } else {
            view! { cx,
                span {
                    a(href="/login") { "Login" }
                }
            }
        };
        provide_context(
            cx,
            data.get("boardTopics").unwrap().as_array().unwrap().clone(),
        );
        view! { cx,
            nav(style="display: flex; justify-content: space-between;") {
                (left)
                (right)
            }
            IndexTopics {}
            (if logged_in { view! {cx, NewTopic {}}} else {view! {cx, }})
        }
    } else {
        view! { cx, "Internal Server Error"}
    }
}

#[component]
pub fn Index<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        Suspense {
            fallback: view! { cx, "Loading..." },
            IndexPage {}
        }
    }
}
