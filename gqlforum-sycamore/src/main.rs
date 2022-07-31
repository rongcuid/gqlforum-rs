pub mod graphql;

use routes::{AppRoutes, TestApp};
use serde::{Deserialize, Serialize};
use sycamore::{prelude::*, suspense::Suspense};
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

use crate::graphql::GraphQLClient;

mod routes;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
    role: String,
}

#[component]
fn App<G: Html>(cx: Scope<'_>) -> View<G> {
    let client = GraphQLClient::new("/graphql");
    provide_context(cx, client);
    view! { cx,
        Router {
            integration: HistoryIntegration::new(),
            view: |cx, route: &ReadSignal<AppRoutes>| {
                view! { cx,
                    div(class="app") {
                        (match route.get().as_ref() {
                            AppRoutes::Index => view! { cx, "Stub index"},
                            AppRoutes::Login => view! { cx, "Stub login"},
                            AppRoutes::Logout => view! { cx, "Stub logout"},
                            AppRoutes::Topic{ .. } => view! { cx, "Stub topic"},
                            AppRoutes::User{ .. } => view! {cx, "Stub user"},
                            AppRoutes::Test => view! { cx, TestApp {}},
                            AppRoutes::NotFound => view! { cx, "404 Not Found"}
                        })
                    }
                }
            }
        }
    }
}

fn main() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();

    sycamore::render(|cx| {
        view! { cx, App {} }
    });
}
