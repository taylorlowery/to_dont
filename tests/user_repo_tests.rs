use to_dont::models::{User, UserDTO};
use to_dont::repository::sqlite::user_repository;

const TEST_CONN_STRING: &str = "./test_db.db3";

#[cfg(test)]
mod tests {
    use to_dont::repository::Repository;

    use super::*;

    #[test]
    fn test_new_user() -> Result<(), rusqlite::Error> {
        let user_repo = user_repository::UserRepository::new(TEST_CONN_STRING)?;

        user_repo.create_db()?;

        let new_user = UserDTO {
            first_name: "Taylor".to_string(),
            last_name: "Lowery".to_string(),
            email: "tlowery@fakemail.com".to_string(),
        };

        let user_id = user_repo.save_new_item(&new_user)?;

        let user: User = user_repo.select_item_by_id(&user_id)?;

        assert_eq!(user.first_name, new_user.first_name);
        assert_eq!(user.last_name, new_user.last_name);
        assert_eq!(user.email, new_user.email);

        Ok(())
    }
}
