use color_eyre::eyre;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use uuid::Uuid as CrateUuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Uuid(String);

impl Uuid {
    pub fn new() -> Self {
        Self(CrateUuid::new_v4().to_string())
    }
}

impl Default for Uuid {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Uuid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<&str> for Uuid {
    type Error = uuid::Error;

    fn try_from(s: &str) -> eyre::Result<Self, Self::Error> {
        let parsed = CrateUuid::try_parse(s)?;

        Ok(Self(parsed.to_string()))
    }
}

impl TryFrom<String> for Uuid {
    type Error = uuid::Error;

    fn try_from(s: String) -> eyre::Result<Self, Self::Error> {
        let parsed = CrateUuid::try_parse(&s)?;

        Ok(Self(parsed.to_string()))
    }
}

impl From<Uuid> for String {
    fn from(u: Uuid) -> Self {
        u.0
    }
}
