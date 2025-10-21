pub enum AuthAPIError {
    IncorrectCredentials,
    InvalidCredentials,
    UnexpectedError,
    UserAlreadyExists,
    MissingToken,
    InvalidToken,
}
