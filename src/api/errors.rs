pub enum ControllerError {
    ValidationError,
    NotFound,
    WrongActionParameters,
    CantCallActionLink,
    SqlError(diesel::result::Error)
}

impl From<diesel::result::Error> for ControllerError {
    fn from(error: diesel::result::Error) -> Self {
        ControllerError::SqlError(error)
    }
}

