use dioxus::prelude::*;
use log::info;

use crate::backend::{self, TaskItem};

#[component]
pub fn TaskView() -> Element {
    rsx! {
        div { id: "task-view",
            TaskInput {}
            TodoList {}
        }
    }
}

#[component]
pub fn TaskInput() -> Element {
    rsx! {
        // form { onsubmit: move |event| { log::info!("Submitted! {event:?}") },
        //     input { name: "title" }
        //     input { r#type: "submit" }
        // }
    }
}

#[component]
pub fn TodoList() -> Element {
    let mut resource = use_resource(backend::get_tasks);
    let tasks = resource.suspend()?;

    rsx! {
        div { id: "todo-list",
            for task in tasks().unwrap() {
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
