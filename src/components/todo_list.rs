use crate::components::{Filter, TodoItem};
use crate::model::Todo;
use crate::server_fns::toggle_all_todos;
use leptos::prelude::*;

#[component]
pub fn TodoMain(
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
    filter: RwSignal<Filter>,
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

    let filtered_todos = Memo::new(move |_| {
        let list = todos_list.get();
        match filter.get() {
            Filter::All => list,
            Filter::Active => list.into_iter().filter(|t| !t.completed).collect(),
            Filter::Completed => list.into_iter().filter(|t| t.completed).collect(),
        }
    });

    let on_toggle_all = move |_| {
        let set_completed = !all_completed.get();
        leptos::task::spawn_local(async move {
            let _ = toggle_all_todos(set_completed).await;
            todos.refetch();
        });
    };

    let has_todos = Memo::new(move |_| !todos_list.get().is_empty());

    view! {
        <Show when=move || has_todos.get()>
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
                        each=move || filtered_todos.get()
                        key=|todo| todo.id
                        let:todo
                    >
                        <TodoItem todo=todo todos=todos/>
                    </For>
                </ul>
            </section>
        </Show>
    }
}
