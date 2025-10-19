#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Email, EmailError> {
        if !email.contains("@") || email.is_empty() {
            Err(EmailError::EmailParseError)
        } else {
            Ok(Self(email))
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}
#[derive(Debug)]
pub enum EmailError {
    EmailParseError,
}
