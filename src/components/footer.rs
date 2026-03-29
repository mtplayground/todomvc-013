use crate::components::Filter;
use crate::model::Todo;
use crate::server_fns::clear_completed;
use leptos::prelude::*;

#[component]
pub fn TodoFooter(
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
    filter: RwSignal<Filter>,
) -> impl IntoView {
    let todos_list = Memo::new(move |_| {
        todos
            .get()
            .and_then(|r| r.ok())
            .unwrap_or_default()
    });

    let has_todos = Memo::new(move |_| !todos_list.get().is_empty());

    let active_count = Memo::new(move |_| {
        todos_list.get().iter().filter(|t| !t.completed).count()
    });

    let has_completed = Memo::new(move |_| {
        todos_list.get().iter().any(|t| t.completed)
    });

    let on_clear_completed = move |_| {
        leptos::task::spawn_local(async move {
            let _ = clear_completed().await;
            todos.refetch();
        });
    };

    view! {
        <Show when=move || has_todos.get()>
            <footer class="footer">
                <span class="todo-count">
                    <strong>{move || active_count.get()}</strong>
                    {move || if active_count.get() == 1 { " item left" } else { " items left" }}
                </span>
                <ul class="filters">
                    <li>
                        <a
                            class:selected=move || filter.get() == Filter::All
                            href="#/"
                            on:click=move |_| filter.set(Filter::All)
                        >"All"</a>
                    </li>
                    <li>
                        <a
                            class:selected=move || filter.get() == Filter::Active
                            href="#/active"
                            on:click=move |_| filter.set(Filter::Active)
                        >"Active"</a>
                    </li>
                    <li>
                        <a
                            class:selected=move || filter.get() == Filter::Completed
                            href="#/completed"
                            on:click=move |_| filter.set(Filter::Completed)
                        >"Completed"</a>
                    </li>
                </ul>
                <Show when=move || has_completed.get()>
                    <button class="clear-completed" on:click=on_clear_completed>
                        "Clear completed"
                    </button>
                </Show>
            </footer>
        </Show>
    }
}
