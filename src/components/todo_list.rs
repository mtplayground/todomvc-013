use crate::components::TodoItem;
use crate::model::Todo;
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

    view! {
        <section class="main">
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
