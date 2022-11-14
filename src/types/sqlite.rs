use std::borrow::{Borrow, Cow};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ErrorCode {
    #[error("unique constriant violation")]
    UniqueConstraintViolation,

    #[error("unknown error code `{0}`")]
    Unknown(String),
}

impl From<Cow<'_, str>> for ErrorCode {
    fn from(code: Cow<'_, str>) -> Self {
        match code.borrow() {
            "2067" => Self::UniqueConstraintViolation,
            c => Self::Unknown(c.to_string()),
        }
    }
}
