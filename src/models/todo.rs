use chrono::{DateTime, Utc};

#[derive(Debug)]
struct TodoItem {
    pub id: i64,
    pub user_id: i64,
    pub task: String,
    pub completed: bool,
    pub created_datetime: DateTime<Utc>,
    pub completed_datetime: DateTime<Utc>,
    // Future: Steps of sub-tasks?
    // pub sub_tasks: Vec<TodoItem>,
    // Future: additional notes?
    // pub notes: Vec<String> // note data type with id and dates?
}

#[derive(Debug)]
struct TodoItemDTO {
    user_id: i64,
    task: String,
}

