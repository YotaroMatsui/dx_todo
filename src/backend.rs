use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::logger::tracing;
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaskItem {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        let conn = rusqlite::Connection::open("todo.db").unwrap_or_else(|err| {
            eprint!("Failed to open database: {}", err);
            panic!("Database initialization failed");
        });

        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS tasks (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                completed BOOLEAN NOT NULL
            )
            "#,
        ).unwrap_or_else(|err| {
            eprint!("Failed to create table: {}", err);
            panic!("Database initialization failed");
        });

        conn
   };
}

#[server]
pub async fn get_tasks() -> Result<Vec<TaskItem>, ServerFnError> {
    let result: rusqlite::Result<Vec<TaskItem>> = DB.with(|db| {
        let mut stmt = db.prepare("SELECT * FROM tasks")?;
        let x = stmt
            .query_map([], |row| {
                Ok(TaskItem {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get(2)?,
                })
            })?
            .collect();
        x
    });

    result.map_err(|reason| {
        tracing::error!(?reason, "failed to fetch tasks");
        ServerFnError::ServerError("failed to fetch tasks from db!".into())
    })
}

#[server]
pub async fn add_task(title: String) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute(
            "INSERT INTO tasks (title, completed) VALUES (?1, ?2)",
            &[&title, &"false".to_string()],
        ) {
            return Err(ServerFnError::new(err.to_string()));
        }
        Ok(())
    })
}

#[server]
pub async fn toggle_task(id: i32) -> Result<(), ServerFnError> {
    DB.with(|db| {
        if let Err(err) = db.execute(
            "UPDATE tasks SET completed = NOT completed WHERE id = ?1",
            &[&id],
        ) {
            return Err(ServerFnError::new(err.to_string()));
        }
        Ok(())
    })
}
