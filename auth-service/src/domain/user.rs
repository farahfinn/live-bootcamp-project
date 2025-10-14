// The User struct shoudl contain 3 fields.  email, which is a String;
// pssword, also a String; and requires_2fa, whih is a boolean
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool,
}

impl User {
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa,
        }
    }
}
