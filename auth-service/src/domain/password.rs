#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Self, PasswordError> {
        if password.len() < 8 {
            Err(PasswordError::PasswordParseError)
        } else {
            Ok(Self(password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug)]
pub enum PasswordError {
    PasswordParseError,
}
