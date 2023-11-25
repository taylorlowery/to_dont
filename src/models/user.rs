#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug)]
pub struct UserDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
