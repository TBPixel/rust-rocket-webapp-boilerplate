use crate::types::uuid::Uuid;

use color_eyre::eyre;
use lazy_regex::regex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn new(s: &str) -> eyre::Result<Self> {
        // note: I hate regex, but this is _probably_ valid enough
        let expr = regex!(r#"^[a-zA-Z0-9!#$%&'*+\-/=?^_`{|}~.]+@[a-zA-Z0-9]+\.[a-zA-Z0-9]+$"#);
        if !expr.is_match(s) {
            return Err(eyre::eyre!("invalid email format"));
        }

        Ok(Self(s.to_lowercase()))
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Email {
    type Error = eyre::Report;

    fn try_from(s: &str) -> eyre::Result<Self> {
        Self::new(s)
    }
}

impl TryFrom<String> for Email {
    type Error = eyre::Report;

    fn try_from(s: String) -> eyre::Result<Self> {
        Self::new(&s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub user_id: Uuid,
    pub email: Email,
}
