
#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    use to_dont::models::todo::{TodoItem, TodoItemDTO};
    use to_dont::repository::Repository;
    use to_dont::repository::sqlite::todo_repository;

    #[test]
    fn test_new_todo_item() -> Result<(), rusqlite::Error> {
        let todo_repo = todo_repository::TodoRepository::new(None)?;

        // create a todo item
        let new_todo_item = TodoItemDTO {
            user_id: 1,
            task: "Test todo item".to_string(),
        };

        // save the todo item
        let todo_id = todo_repo.save_new_item(&new_todo_item)?;

        // retrieve the todo item by id
        let todo_item: TodoItem = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the retrieved todo item data has the correct values
        assert_eq!(todo_item.user_id, new_todo_item.user_id);
        assert_eq!(todo_item.task, new_todo_item.task);

        Ok(())
    }

    #[test]
    fn test_update_todo_item() -> Result<(), rusqlite::Error> {
        let todo_repo = todo_repository::TodoRepository::new(None)?;

        // create a todo item
        let new_todo_item = TodoItemDTO {
            user_id: 1,
            task: "Test todo item".to_string(),
        };

        // save the todo item
        let todo_id = todo_repo.save_new_item(&new_todo_item)?;
        let mut todo_item: TodoItem = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the todo item has the correct values
        assert_eq!(todo_item.user_id, new_todo_item.user_id);
        assert_eq!(todo_item.task, new_todo_item.task);

        // create an update dto
        let update_dto = TodoItemDTO {
            user_id: 1,
            task: "Updated todo item".to_string(),
        };

        // update the todo item
        todo_repo.update_item(&todo_id, &update_dto)?;

        // retrieve the todo item by id
        todo_item = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the retrieved todo item data has the correct values
        assert_eq!(todo_item.user_id, update_dto.user_id);
        assert_eq!(todo_item.task, update_dto.task);

        Ok(())
    }

    #[test]
    fn test_complete_and_uncomplete_todo_item() -> Result<(), rusqlite::Error> {
        let todo_repo = todo_repository::TodoRepository::new(None)?;

        // create a todo item
        let new_todo_item = TodoItemDTO {
            user_id: 1,
            task: "Test todo item".to_string(),
        };

        // save the todo item
        let todo_id = todo_repo.save_new_item(&new_todo_item)?;
        let mut todo_item: TodoItem = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the todo item has the correct values
        assert_eq!(todo_item.user_id, new_todo_item.user_id);
        assert_eq!(todo_item.task, new_todo_item.task);
        assert_eq!(todo_item.completed, false);

        // complete the todo item
        todo_repo.complete_todo_item(&todo_id)?;

        // retrieve the todo item by id
        todo_item = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the retrieved todo item data has the correct values
        assert_eq!(todo_item.user_id, new_todo_item.user_id);
        assert_eq!(todo_item.task, new_todo_item.task);
        assert_eq!(todo_item.completed, true);

        // uncomplete the todo item
        todo_repo.uncomplete_todo_item(&todo_id)?;

        // retrieve the todo item by id
        todo_item = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the retrieved todo item data has the correct values
        assert_eq!(todo_item.user_id, new_todo_item.user_id);
        assert_eq!(todo_item.task, new_todo_item.task);
        assert_eq!(todo_item.completed, false);

        Ok(())
    }

    #[test]
    fn test_delete_todo_item() -> Result<(), rusqlite::Error> {
        let todo_repo = todo_repository::TodoRepository::new(None)?;

        // create a todo item
        let new_todo_item = TodoItemDTO {
            user_id: 1,
            task: "Test todo item".to_string(),
        };

        // save the todo item
        let todo_id = todo_repo.save_new_item(&new_todo_item)?;

        // created another todo item
        let new_todo_item_2 = TodoItemDTO {
            user_id: 2,
            task: "Test todo item 2".to_string(),
        };

        // save the todo item
        let todo_id_2 = todo_repo.save_new_item(&new_todo_item_2)?;

        // retrieve the todo item by id
        let todo_item: TodoItem = todo_repo.select_item_by_id(&todo_id)?;

        // make sure the retrieved todo item data has the correct values
        assert_eq!(todo_item.user_id, new_todo_item.user_id);
        assert_eq!(todo_item.task, new_todo_item.task);

        // delete the todo item
        todo_repo.delete_item_by_id(&todo_id)?;

        // try to retrieve the todo item by id
        let result = todo_repo.select_item_by_id(&todo_id);

        // make sure the todo item was not found
        assert!(result.is_err());

        // make sure the second todo item still exists
        let todo_item_2: TodoItem = todo_repo.select_item_by_id(&todo_id_2)?;

        // make sure the retrieved todo item data has the correct values
        assert_eq!(todo_item_2.user_id, new_todo_item_2.user_id);
        assert_eq!(todo_item_2.task, new_todo_item_2.task);

        Ok(())
    }

    #[test]
    fn test_get_user_todos() -> Result<(), rusqlite::Error> {
        let todo_repo = todo_repository::TodoRepository::new(None)?;

        // create a todo item
        let user_1_new_todo_item = TodoItemDTO {
            user_id: 1,
            task: "Test todo item".to_string(),
        };

        // save the todo item
        let user_1_todo_id_1 = todo_repo.save_new_item(&user_1_new_todo_item)?;

        // create another todo item
        let user_1_new_todo_item_2 = TodoItemDTO {
            user_id: 1,
            task: "Test todo item 2".to_string(),
        };

        // save the todo item
        let user_1_todo_id_2 = todo_repo.save_new_item(&user_1_new_todo_item_2)?;

        // create a todo item for user 2
        let user_2_new_todo_item = TodoItemDTO {
            user_id: 2,
            task: "Test todo item".to_string(),
        };

        // save the todo item
        let user_2_todo_id = todo_repo.save_new_item(&user_2_new_todo_item)?;

        // get user 1 todos
        let user_1_todos = todo_repo.get_user_todos(&1)?;

        // make sure user 1 has 2 todos
        assert_eq!(user_1_todos.len(), 2);

        // make sure both todos have the correct values
        assert_eq!(user_1_todos[0].id, user_1_todo_id_1);
        assert_eq!(user_1_todos[0].user_id, user_1_new_todo_item.user_id);
        assert_eq!(user_1_todos[0].task, user_1_new_todo_item.task);

        assert_eq!(user_1_todos[1].id, user_1_todo_id_2);
        assert_eq!(user_1_todos[1].user_id, user_1_new_todo_item_2.user_id);
        assert_eq!(user_1_todos[1].task, user_1_new_todo_item_2.task);

        // just to be sure, make sure none of user 1's todos have the same id as user 2's todo
        assert_ne!(user_1_todos[0].id, user_2_todo_id);
        assert_ne!(user_1_todos[1].id, user_2_todo_id);

        Ok(())
    }

    #[test]
    fn test_create_db_from_todo_repo() -> Result<(), Box<dyn Error>> {

        let test_conn_string: &str = "./todo_test_db.db3";

        // make sure that the test db file does not exist
        if Path::new(test_conn_string).exists() {
            fs::remove_file(test_conn_string)?;
        }
        let db_exists = Path::new(test_conn_string).exists();
        assert_eq!(db_exists, false);

        // create scope to hold the db connection
        // so that it will be dropped and the db file will be closed
        // otherwise, the file will be locked and the test will panic when we attempt to delete it
        {
            // create the test db
            let _ = todo_repository::TodoRepository::new(Some(test_conn_string))?;

            // validate the that the db file exists
            let db_exists = Path::new(test_conn_string).exists();
            assert_eq!(db_exists, true);
        }

        // delete the test db
        fs::remove_file(test_conn_string)?;

        // just to be thorough, make sure the file was deleted
        let db_exists = Path::new(test_conn_string).exists();
        assert_eq!(db_exists, false);

        Ok(())
    }
}