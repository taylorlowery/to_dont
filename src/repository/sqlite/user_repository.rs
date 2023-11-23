use rusqlite::{Connection, params, Result};

use crate::models::{User, UserDTO};
use crate::repository::entity::Entity;
use crate::repository::Repository;

pub struct UserRepository {
    conn: Connection,
}

impl Entity for User {
    type Id = i64;
    type Item = User;
    type ItemDto = UserDTO;
}

impl UserRepository {
    /// Generate an instance of the user repository.
    /// If no connection string (desired db file name) is provided, returns an in-memory db.
    pub fn new(connection_string: Option<&str>) -> Result<UserRepository> {
        let conn = match connection_string {
            Some(connection_string) => UserRepository::connect_to_db(connection_string)?,
            None => Connection::open_in_memory()?,
        };
        Ok(UserRepository { conn })
    }
}


impl Repository<Connection, User, rusqlite::Error> for UserRepository {
    fn connect_to_db(connection_string: &str) -> Result<Connection> {
        let conn: Connection = Connection::open(connection_string)?;
        Ok(conn)
    }
    /// Create the SQLite database structure for the blog database
    fn create_db(&self) -> Result<()> {
        // create users table
        self.conn.execute(
            "CREATE TABLE users (\
id INTEGER PRIMARY KEY,\
first_name TEXT NOT NULL,\
last_name TEXT NOT NULL,\
email TEXT NOT NULL\
)",
            (),
        )?;
//
//         // create credentials table
//         conn.execute(
//             "CREATE TABLE credentials(\
// email TEXT PRIMARY KEY,
// password TEXT NOT NULL
// )", ()
//         )?;
//
//         // create blog posts table
//         conn.execute(
//             "CREATE TABLE posts(\
// id INTEGER PRIMARY KEY,\
// created_datetime INTEGER DEFAULT (strftime('%s', 'now')),\
// title TEXT NOT NULL,\
// text TEXT NOT NULL\
// )",
//             (),
//         )?;

        Ok(())
    }
    fn save_new_item(&self, user_dto: &UserDTO) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO users (first_name, last_name, email) VALUES (?1, ?2, ?3)",
            params![user_dto.first_name, user_dto.last_name, user_dto.email],
        )?;
        Ok(self.conn.last_insert_rowid())
    }
    fn select_item_by_id(&self, id: &i64) -> Result<User> {
        self.conn.query_row(
            "Select id, first_name, last_name, email FROM users where id = ?1",
            params![id],
            |row| {
                Ok(User {
                    id: row.get(0)?,
                    first_name: row.get(1)?,
                    last_name: row.get(2)?,
                    email: row.get(3)?,
                })
            },
        ).map_err(|e| e.into())
    }
    fn update_item(&self, user: &User) -> Result<usize> {
        let updated_count = self.conn.execute(
            "UPDATE users SET first_name = ?1, last_name = ?2, email = ?3 WHERE id = ?4",
            params![user.first_name, user.last_name, user.email, user.id],
        )?;
        Ok(updated_count)
    }
    fn delete_item_by_id(&self, id: &i64) -> Result<usize> {
        let deleted_count = self.conn.execute(
            "DELETE FROM users WHERE id = ?1",
            params![id],
        )?;
        Ok(deleted_count)
    }
}
