use gloo_timers::future::TimeoutFuture;
use serde_json::Value;
use sycamore::{futures::spawn_local_scoped, prelude::*, suspense::Suspense};
use sycamore_router::navigate;

use crate::graphql::GraphQLClient;

#[component]
async fn LogoutOutput<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let client = use_context::<GraphQLClient>(cx);
    let resp = client
        .query_raw(
            r#"
                    mutation {
                        logout
                    }
                    "#,
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
            if (|| {
                Some(data.get("logout")? )
            })() == Some(&Value::Bool(true)) {
                spawn_local_scoped(cx, async move {
                    TimeoutFuture::new(1000).await;
                    navigate("/");
                });
                view! {cx, p {"Logged out! Redirecting in 1 second..."} }
            } else {
                view! {cx, p {"Authentication error!"}}
            }
        } else {
            view! {cx, p {"Internal Server Error"}}
        }
    )}
}

#[component]
pub fn Logout<G: Html>(cx: Scope<'_>) -> View<G> {
    view! { cx,
        Suspense {
            fallback: view! { cx, "Logging out..." },
            LogoutOutput { }
        }
    }
}
