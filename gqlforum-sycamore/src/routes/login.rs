use serde_json::json;
use sycamore::{prelude::*, suspense::Suspense};

use crate::graphql::GraphQLClient;

struct Username(String);
struct Password(String);

#[component]
async fn LoginOutput<'a, G: Html>(cx: Scope<'a>) -> View<G> {
    let username = use_context::<ReadSignal<Username>>(cx);
    let password = use_context::<ReadSignal<Password>>(cx);
    let client = use_context::<GraphQLClient>(cx);
    let resp = client
        .query_raw_with(
            r#"
                    mutation ($username: String, $password: String) {
                        login(username: $username, password: $password)
                    }
                    "#,
            json!({
                "username": username.get().0,
                "password": password.get().0,
            }),
        )
        .await
        .unwrap();
    view! {cx, (format!("{:?}", resp))}
}

#[component]
pub fn Login<G: Html>(cx: Scope<'_>) -> View<G> {
    let submitted = create_signal(cx, false);
    let username = create_signal(cx, String::new());
    let password = create_signal(cx, String::new());
    provide_context_ref(cx, create_memo(cx, || Username((*username.get()).clone())));
    provide_context_ref(cx, create_memo(cx, || Password((*password.get()).clone())));
    let login = |_| {
        submitted.set(true);
    };
    view! { cx,
        form {
            div {
                label { "Username: " }
                input(type="text", placeholder="Enter Username", name="username", bind:value=username) {}
            }
            div {
                label { "Password: " }
                input(type="password", placeholder="Enter Password", name="password",bind:value=password) {}
            }
            div {
                button(on:click=login,type="button") { "Login" }
            }
            div {
            (if *submitted.get() { view! { cx,
                    Suspense {
                        fallback: view! {cx, "Logging in..."},
                        LoginOutput {}
                    }
                } } else { view! {cx, } })
            }
        }
    }
}
