use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::{Connection, Error, params, Result};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};

use crate::models::{TodoItem, TodoItemDTO};
use crate::repository::entity::Entity;
use crate::repository::Repository;

pub struct TodoRepository {
    conn: Connection,
}

impl Entity for TodoItem {
    type Id = i64;
    type Item = TodoItem;
    type ItemDto = TodoItemDTO;
}

impl TodoRepository {
    pub fn new(connection_string: Option<&str>) -> Result<TodoRepository> {
        let conn = match connection_string {
            None => Connection::open_in_memory()?,
            Some(connection_string) => TodoRepository::connect_to_db(connection_string)?,
        };
        let todo_repo = TodoRepository { conn };
        todo_repo.create_db()?;
        Ok(todo_repo)
    }

    fn create_db(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE todos(\
id INTEGER PRIMARY KEY,\
user_id INTEGER NOT NULL,\
task TEXT NOT NULL,\
completed INTEGER NOT NULL DEFAULT 0,\
created_datetime INTEGER DEFAULT (strftime('%s', 'now')),\
completed_datetime INTEGER)",
            (),
        )?;
        Ok(())
    }

    pub fn get_user_todos(&self, user_id: &i64) -> Result<Vec<TodoItem>> {
        let mut stmt = self.conn.prepare("SELECT * FROM todos WHERE user_id = ?1")?;
        let todo_iter = stmt.query_map(params![user_id], |row| {
            Ok(TodoItem {
                id: row.get(0)?,
                user_id: row.get(1)?,
                task: row.get(2)?,
                completed: row.get(3)?,
                created_datetime: DateTime::from_timestamp(row.get(4)?, 0).ok_or(rusqlite::Error::QueryReturnedNoRows)?,
                completed_datetime: DateTime::from_timestamp(row.get(5)?, 0).ok_or(rusqlite::Error::QueryReturnedNoRows)?,
            })
        })?;
        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo?);
        }
        Ok(todos)
    }

    pub fn complete_todo_item(&self, id: &i64) -> Result<usize> {
        self.conn.execute(
            "UPDATE todos SET completed = 1, completed_datetime = (strftime('%s', 'now')) WHERE id = ?1",
            params![id],
        )
    }

    pub fn uncomplete_todo_item(&self, id: &i64) -> Result<usize> {
        self.conn.execute(
            "UPDATE todos SET completed = 0, completed_datetime = NULL WHERE id = ?1",
            params![id],
        )
    }
}

impl Repository<Connection, TodoItem, rusqlite::Error> for TodoRepository {
    fn connect_to_db(connection_string: &str) -> Result<Connection> {
        let conn: Connection = Connection::open(connection_string)?;
        Ok(conn)
    }

    fn save_new_item(&self, todo_dto: &TodoItemDTO) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO todos (user_id, task) VALUES (?1, ?2)",
            params![todo_dto.user_id, todo_dto.task],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    fn select_item_by_id(&self, id: &i64) -> Result<TodoItem> {
        self.conn.query_row(
            "Select id, user_id, task, completed, created_datetime, completed_datetime FROM todos where id = ?1",
            params![id],
            |row| {
                Ok(TodoItem {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    task: row.get(2)?,
                    completed: row.get(3)?,
                    created_datetime: DateTime::from_timestamp(row.get(4)?, 0).ok_or(rusqlite::Error::QueryReturnedNoRows)?,
                    completed_datetime: DateTime::from_timestamp(row.get(5)?, 0).ok_or(rusqlite::Error::QueryReturnedNoRows)?,
                })
            },
        ).map_err(|e| e.into())
    }

    fn update_item(&self, id: &i64, todo_item: &TodoItemDTO) -> Result<usize> {
        self.conn.execute("UPDATE todos SET task = ?1 WHERE id = ?2",
            params![todo_item.task, id],
        )
    }

    fn delete_item_by_id(&self, id: &i64) -> Result<usize> {
        self.conn.execute(
            "DELETE FROM todos WHERE id = ?1",
            params![id],
        )
    }
}

