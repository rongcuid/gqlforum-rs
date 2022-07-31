use sycamore::builder::prelude::*;
use sycamore::prelude::*;
use sycamore::suspense::Suspense;

use crate::graphql::GraphQLClient;

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
            meta {
                title
                author {
                    name
                }
            }
            createdAt
            updatedAt
        }
    }
    "#,
        )
        .await
        .unwrap();
    let left = view! { cx,
            span {a(href="/") { "Home" } }
    };
    if let Some(data) = resp.data {
        let right = if let Some(user) = data.get("session").unwrap().get("user") {
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
        view! { cx,
            nav(style="display: flex; justify-content: space-between;") {
                (left)
                (right)
            }
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
