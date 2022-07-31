use sycamore::prelude::*;

#[component]
pub async fn NewTopic<G: Html>(cx: Scope<'_>) -> View<G> {
    let title = create_signal(cx, String::new());
    let content = create_signal(cx, String::new());
    let new_topic = |_| {};
    view! {
        cx,
        form {
            div {
                input(type="text", bind:value=title)
            }
            div {
                textarea(type="text", bind:value=content)
            }
            div {
                button(on:click=new_topic,type="button") { "New Topic" }
            }
        }
    }
}