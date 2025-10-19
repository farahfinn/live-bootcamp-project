use crate::domain::{email::Email, password::Password};

// The User struct shoudl contain 3 fields.  email, which is a String;
// pssword, also a String; and requires_2fa, whih is a boolean
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct User {
    pub email: Email,
    pub password: Password,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}
