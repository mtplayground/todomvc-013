use crate::model::Todo;
use crate::server_fns::{delete_todo, toggle_todo, update_todo_title};
use leptos::prelude::*;

#[component]
pub fn TodoItem(
    todo: Todo,
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
) -> impl IntoView {
    let id = todo.id;
    let completed = todo.completed;
    let title = todo.title.clone();

    let (editing, set_editing) = signal(false);
    let edit_ref = NodeRef::<leptos::html::Input>::new();

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

    let save_edit = move || {
        if !editing.get_untracked() {
            return;
        }
        set_editing.set(false);
        let new_title = edit_ref
            .get()
            .map(|input| input.value().trim().to_string())
            .unwrap_or_default();
        if new_title.is_empty() {
            leptos::task::spawn_local(async move {
                let _ = delete_todo(id).await;
                todos.refetch();
            });
        } else {
            leptos::task::spawn_local(async move {
                let _ = update_todo_title(id, new_title).await;
                todos.refetch();
            });
        }
    };

    let on_dblclick = {
        let title = title.clone();
        move |_| {
            set_editing.set(true);
            if let Some(input) = edit_ref.get() {
                input.set_value(&title);
            }
            request_animation_frame(move || {
                if let Some(input) = edit_ref.get() {
                    let _ = input.focus();
                }
            });
        }
    };

    let on_edit_keydown = move |ev: leptos::ev::KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => save_edit(),
            "Escape" => set_editing.set(false),
            _ => {}
        }
    };

    let on_edit_blur = move |_| {
        save_edit();
    };

    view! {
        <li class:completed=completed class:editing=move || editing.get()>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=on_toggle
                />
                <label on:dblclick=on_dblclick>{title}</label>
                <button class="destroy" on:click=on_destroy></button>
            </div>
            <input
                class="edit"
                node_ref=edit_ref
                on:keydown=on_edit_keydown
                on:blur=on_edit_blur
            />
        </li>
    }
}
