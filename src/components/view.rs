use std::task;

use dioxus::{logger::tracing, prelude::*};

use crate::backend::{self, TaskItem};

#[component]
pub fn TaskView() -> Element {
    rsx! {
        div { id: "task-view",
            TaskInput {}
            SuspenseBoundary { fallback: |_| rsx! { "Loading Todolist..." }, TodoList {} }
        }
    }
}

#[component]
pub fn TaskInput() -> Element {
    rsx! {
        form { onsubmit: move |event| { tracing::info!("Submitted! {event:?}") },
            input { name: "title" }
            input { r#type: "submit" }
        }
    }
}

#[component]
pub fn TodoList() -> Element {
    let mut resource = use_resource(backend::get_tasks);
    let tasks = resource.suspend()?.read().clone();

    match tasks {
        Ok(tasks) => {
            rsx! {
                div { id: "todo-list",
                    for task in tasks {
                        div { key: task.id, class: "task-item",
                            input {
                                r#type: "checkbox",
                                checked: task.completed,
                                onclick: move |_| {
                                    async move {
                                        if let Err(err) = backend::toggle_task(task.id).await {
                                            eprintln!("Failed to toggle task: {}", err);
                                        } else {
                                            resource.restart();
                                        }
                                    }
                                },
                            }
                            "{task.title}"
                        }
                    }
                }
            }
        }
        Err(err) => {
            rsx! {
                div { "Failed to fetch tasks: {err}" }
            }
        }
    }
}
