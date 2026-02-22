use crate::core::validation::InputValidationError;
use crate::core::rules::tax_rules::TaxRuleSelectionError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EngineError {
    Validation(InputValidationError),
    RuleSelection(TaxRuleSelectionError),
    Computation(String),
}

impl fmt::Display for EngineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EngineError::Validation(err) => write!(f, "{err}"),
            EngineError::RuleSelection(err) => write!(f, "{err}"),
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

impl From<TaxRuleSelectionError> for EngineError {
    fn from(value: TaxRuleSelectionError) -> Self {
        EngineError::RuleSelection(value)
    }
}
