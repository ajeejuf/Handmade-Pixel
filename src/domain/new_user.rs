use crate::domain::UserEmail;
use crate::domain::Username;

pub struct NewUser {
    pub email: UserEmail,
    pub username: Username,
    pub password: String
}