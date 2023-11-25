
#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::fs;
    use std::path::Path;

    use to_dont::models::{User, UserDTO};
    use to_dont::repository::Repository;
    use to_dont::repository::sqlite::user_repository;

    #[test]
    fn test_new_user() -> Result<(), rusqlite::Error> {
        let user_repo = user_repository::UserRepository::new(None)?;

        // create a user
        let new_user = UserDTO {
            first_name: "Taylor".to_string(),
            last_name: "Lowery".to_string(),
            email: "tlowery@fakemail.com".to_string(),
        };

        // save the user
        let user_id = user_repo.save_new_item(&new_user)?;

        // retrieve the user by id
        let user: User = user_repo.select_item_by_id(&user_id)?;

        // make sure the retrieved user data has the correct values
        assert_eq!(user.first_name, new_user.first_name);
        assert_eq!(user.last_name, new_user.last_name);
        assert_eq!(user.email, new_user.email);

        Ok(())
    }

    #[test]
    fn test_update_user() -> Result<(), rusqlite::Error> {
        let user_repo = user_repository::UserRepository::new(None)?;

        // create a user
        let new_user = UserDTO {
            first_name: "Taylor".to_string(),
            last_name: "Lowery".to_string(),
            email: "tlowery@fakemail.com".to_string(),
        };

        // save the user
        let user_id = user_repo.save_new_item(&new_user)?;
        let user: User = user_repo.select_item_by_id(&user_id)?;

        // make sure the user has the correct values
        assert_eq!(user.first_name, new_user.first_name);
        assert_eq!(user.last_name, new_user.last_name);
        assert_eq!(user.email, new_user.email);

        // create an update dto
        let update_dto = UserDTO {
            first_name: "Tater".to_string(),
            last_name: "Tot".to_string(),
            email: "2hott2tott@fakemail.com".to_string()
        };

        // update the user in the database
        user_repo.update_item(&user.id, &update_dto)?;

        // retieve the user by id after update
        let updated_user: User = user_repo.select_item_by_id(&user_id)?;

        // make sure the user has the updated values
        assert_eq!(updated_user.first_name, update_dto.first_name);
        assert_eq!(updated_user.last_name, update_dto.last_name);
        assert_eq!(updated_user.email, update_dto.email);

        Ok(())
    }

    #[test]
    fn test_delete_user() -> Result<(), rusqlite::Error> {
        let user_repo = user_repository::UserRepository::new(None)?;

        // create a user
        let new_user = UserDTO {
            first_name: "Taylor".to_string(),
            last_name: "Lowery".to_string(),
            email: "tlowery@fakemail.com".to_string(),
        };

        // save the user
        let user_id = user_repo.save_new_item(&new_user)?;
        let user: User = user_repo.select_item_by_id(&user_id)?;

        // make sure the user has the correct values
        assert_eq!(user.first_name, new_user.first_name);
        assert_eq!(user.last_name, new_user.last_name);
        assert_eq!(user.email, new_user.email);

        // create a second user
        let new_user2 = UserDTO {
            first_name: "Tater".to_string(),
            last_name: "Tot".to_string(),
            email: "2hott2tott@fakemail.com".to_string(),
        };

        let user_id2 = user_repo.save_new_item(&new_user2)?;
        let user2: User = user_repo.select_item_by_id(&user_id2)?;

        // make sure the second user has the correct values
        assert_eq!(user2.first_name, new_user2.first_name);
        assert_eq!(user2.last_name, new_user2.last_name);
        assert_eq!(user2.email, new_user2.email);

        // delete the first user
        user_repo.delete_item_by_id(&user_id)?;

        // make sure the user has been deleted
        let deleted_user = user_repo.select_item_by_id(&user_id);
        assert!(deleted_user.is_err());

        // make sure the other user is still there
        let user2: User = user_repo.select_item_by_id(&user_id2)?;

        // make sure the second user has the correct values
        assert_eq!(user2.first_name, new_user2.first_name);
        assert_eq!(user2.last_name, new_user2.last_name);
        assert_eq!(user2.email, new_user2.email);

        Ok(())
    }


    #[test]
    fn check_db_created_from_user_repo() -> Result<(), Box<dyn Error>> {

        let test_conn_string: &str = "./user_test_db.db3";

        if Path::new(test_conn_string).exists() {
            fs::remove_file(test_conn_string)?;
        }
        // make sure that the test db file does not exist
        let db_exists = Path::new(test_conn_string).exists();
        assert_eq!(db_exists, false);

        // create scope to hold the db connection
        // so that it will be dropped and the db file will be closed
        // otherwise, the file will be locked and the test will panic when we attempt to delete it
        {
            // create the test db
            let _ = user_repository::UserRepository::new(Some(test_conn_string))?;

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
