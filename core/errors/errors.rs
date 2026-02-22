use crate::core::validation::InputValidationError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    Validation(InputValidationError),
    Computation(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Validation(err) => write!(f, "{err}"),
            EngineError::Computation(message) => write!(f, "Computation error: {message}"),
        }
    }
}

impl std::error::Error for EngineError {}

impl From<InputValidationError> for EngineError {
    fn from(value: InputValidationError) -> Self {
        EngineError::Validation(value)
    }
}
