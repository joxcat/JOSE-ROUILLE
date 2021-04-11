use nom::error::{VerboseError, VerboseErrorKind};
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum JoseError {
    #[error("Not a boolean")]
    NotABool,
    #[error("Not a null")]
    NotANull,
    #[error("Not escaped")]
    NotEscaped,
    #[error("Not a string")]
    NotAString,
    #[error("Not an integer")]
    NotAnInteger,
}

impl From<JoseError> for nom::Err<VerboseError<JoseError>> {
    fn from(from: JoseError) -> Self {
        Self::Error(VerboseError {
            errors: vec![(from, VerboseErrorKind::Context("JoseError"))],
        })
    }
}
