use serde_json::{json, Value};
use sycamore::prelude::*;
use sycamore::suspense::Suspense;
use sycamore_router::navigate;

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

struct NewTopicInput {
    title: String,
    body: String,
}

#[component]
async fn NewTopicOutput<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let input = use_context::<ReadSignal<NewTopicInput>>(cx);
    let resp = client
        .query_raw_with(
            r#"
        mutation($title: String, $body: String) {
            newTopic(title: $title, body: $body) {
                id
            }
        }
        "#,
            json!({
                "title": input.get().title,
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
        } else if let Some(data) = &resp.data {
            let id = data.get("newTopic").unwrap().get("id").unwrap().as_i64().unwrap();
            navigate(&format!("/topic/{}/1", id));
            view! { cx, p {"Topic posted"}}
        } else {
            view! {cx, p {"Internal Server Error"}}
        }
    )}
}

#[component]
fn NewTopic<G: Html>(cx: Scope<'_>) -> View<G> {
    let submitted = create_signal(cx, false);
    let title = create_signal(cx, String::new());
    let body = create_signal(cx, String::new());
    provide_context_ref(
        cx,
        create_memo(cx, || NewTopicInput {
            title: (*title.get()).clone(),
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
                input(type="text", bind:value=title)
            }
            div {
                textarea(type="text", bind:value=body)
            }
            div {
                button(on:click=new_topic,type="button") { "New Topic" }
            }
            div {
                (if *submitted.get() { view! { cx,
                        Suspense {
                            fallback: view! {cx, "Posting..."},
                            NewTopicOutput { }
                        }
                    } } else { view! {cx, } })
                }
        }
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
