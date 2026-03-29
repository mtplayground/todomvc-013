use crate::model::Todo;
use crate::server_fns::{delete_todo, toggle_todo};
use leptos::prelude::*;

#[component]
pub fn TodoItem(
    todo: Todo,
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
) -> impl IntoView {
    let id = todo.id;
    let completed = todo.completed;
    let title = todo.title.clone();

    let on_toggle = move |_| {
        leptos::task::spawn_local(async move {
            let _ = toggle_todo(id).await;
            todos.refetch();
        });
    };

    let on_destroy = move |_| {
        leptos::task::spawn_local(async move {
            let _ = delete_todo(id).await;
            todos.refetch();
        });
    };

    view! {
        <li class:completed=completed>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=on_toggle
                />
                <label>{title}</label>
                <button class="destroy" on:click=on_destroy></button>
            </div>
        </li>
    }
}
