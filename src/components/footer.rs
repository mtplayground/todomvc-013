use crate::model::Todo;
use leptos::prelude::*;

#[component]
pub fn TodoFooter(
    todos: Resource<Result<Vec<Todo>, ServerFnError>>,
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

    view! {
        <Show when=move || has_todos.get()>
            <footer class="footer">
                <span class="todo-count">
                    <strong>{move || active_count.get()}</strong>
                    {move || if active_count.get() == 1 { " item left" } else { " items left" }}
                </span>
            </footer>
        </Show>
    }
}
