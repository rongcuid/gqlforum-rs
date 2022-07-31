pub mod graphql;

use routes::*;
use serde::{Deserialize, Serialize};
use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Router};

use crate::graphql::GraphQLClient;

mod routes;
mod components;

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
                            AppRoutes::Index => view! { cx, Index {}},
                            AppRoutes::Login => view! { cx, Login {}},
                            AppRoutes::Logout => view! { cx, Logout {}},
                            AppRoutes::Topic{ .. } => view! { cx, Topic {}},
                            AppRoutes::User{ .. } => view! {cx, User {}},
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
