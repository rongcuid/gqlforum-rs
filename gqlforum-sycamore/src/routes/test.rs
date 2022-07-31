use sycamore::{prelude::*, suspense::Suspense};

use crate::graphql::GraphQLClient;

#[component]
async fn TestGql<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let resp1 = client
        .query_raw(
            r#"
    query {
        user(by: {id: 1}) 
        {
            id 
            name 
            role 
        } 
    }
    "#,
        )
        .await
        .unwrap();
    let resp2 = client.query_raw("{ asdfdasf }").await.unwrap();
    view! { cx,
        p {
            "Response: " (format!("{:?}",resp1))
        }
        p {
            "Error: " (format!("{:?}", resp2))
        }
    }
}

#[component]
async fn TestAsync<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        p { "Hello from async!" }
    }
}

#[component]
pub fn TestApp<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        p { "Hello, World!" }
        Suspense {
            fallback: view! { cx, "Async..." },
            TestAsync {}
        }
        Suspense {
            fallback: view! { cx, "Loading..." },
            TestGql {}
        }
    }
}
