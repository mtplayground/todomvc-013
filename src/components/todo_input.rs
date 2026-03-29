use crate::model::Todo;
use crate::server_fns::add_todo;
use leptos::prelude::*;

#[component]
pub fn TodoHeader(
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
) -> impl IntoView {
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let on_keydown = move |ev: leptos::ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            if let Some(input) = input_ref.get() {
                let value = input.value().trim().to_string();
                if !value.is_empty() {
                    input.set_value("");
                    leptos::task::spawn_local(async move {
                        let _ = add_todo(value).await;
                        todos.refetch();
                    });
                }
            }
        }
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autofocus=true
                node_ref=input_ref
                on:keydown=on_keydown
            />
        </header>
    }
}
