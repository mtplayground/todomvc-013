use crate::components::TodoItem;
use crate::model::Todo;
use crate::server_fns::toggle_all_todos;
use leptos::prelude::*;

#[component]
pub fn TodoMain(
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
) -> impl IntoView {
    let todos_list = Memo::new(move |_| {
        todos
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
    });

    let all_completed = Memo::new(move |_| {
        let list = todos_list.get();
        !list.is_empty() && list.iter().all(|t| t.completed)
    });

    let on_toggle_all = move |_| {
        let set_completed = !all_completed.get();
        leptos::task::spawn_local(async move {
            let _ = toggle_all_todos(set_completed).await;
            todos.refetch();
        });
    };

    view! {
        <section class="main">
            <input
                id="toggle-all"
                class="toggle-all"
                type="checkbox"
                prop:checked=move || all_completed.get()
                on:change=on_toggle_all
            />
            <label for="toggle-all">"Mark all as complete"</label>
            <ul class="todo-list">
                <For
                    each=move || todos_list.get()
                    key=|todo| todo.id
                    let:todo
                >
                    <TodoItem todo=todo todos=todos/>
                </For>
            </ul>
        </section>
    }
}
