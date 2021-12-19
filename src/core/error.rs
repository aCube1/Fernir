use std::fmt::{self, Display, Formatter};

pub type FerResult<T> = Result<T, FerError>;

#[derive(Debug)]
pub enum FerError {
    CoreError(String),
    SceneError(String),
}

impl Display for FerError {
    fn fmt(&self, format: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FerError::CoreError(msg) => {
                write!(format, "[Core ERROR]: {}", msg)
            }
            FerError::SceneError(msg) => {
                write!(format, "[Scene Manager ERROR]: {}", msg)
            }
        }
    }
}
