use thiserror::Error;

#[derive(Debug, Clone, Error)]
#[error("failed to validate {field} with error: {message}")]
pub struct FieldValidationError {
    pub field: String,
    pub message: String,
}
