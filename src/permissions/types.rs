use crate::types::{uuid::Uuid, validation::FieldValidationError};
use crate::permissions;

use color_eyre::eyre;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Resource {
    User(String),
    Tenant(String),
}

impl Resource {
    pub fn id(&self) -> &str {
        match self {
            Resource::User(id) | Resource::Tenant(id) => id,
        }
    }

    pub fn kind(&self) -> permissions::types::Target {
        match self {
            Resource::User(_) => permissions::types::Target("user".to_string()),
            Resource::Tenant(_) => permissions::types::Target("tenant".to_string()),
        }
    }
}

impl TryFrom<(&str, &str)> for Resource {
    type Error = eyre::Report;

    fn try_from(id_kind: (&str, &str)) -> eyre::Result<Self, Self::Error> {
        let (id, kind) = id_kind;
        match kind {
            "user" => Ok(Resource::User(id.to_string())),
            "tenant" => Ok(Resource::Tenant(id.to_string())),
            _ => {
                Err(eyre::eyre!(
                    "invalid `resource_kind` in permission string: {}",
                    kind
                ))
            }
        }
    }
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.id(), self.kind())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target(pub String);

impl Target {
    pub fn new(target: &str) -> eyre::Result<Self> {
        target
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-')
            .then(|| true)
            .ok_or(eyre::eyre!(
                "target string can only contain alphanumerica and '-'"
            ))?;

        Ok(Self(target.to_string()))
    }
}

impl TryFrom<String> for Target {
    type Error = eyre::Report;

    fn try_from(s: String) -> eyre::Result<Self, Self::Error> {
        Self::new(&s)
    }
}

impl TryFrom<&str> for Target {
    type Error = eyre::Report;

    fn try_from(s: &str) -> eyre::Result<Self, Self::Error> {
        Self::new(s)
    }
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Actionable {
    Read(Target),
    Write(Target),
    Execute(Target),
}

impl TryFrom<&str> for Actionable {
    type Error = eyre::Report;

    fn try_from(s: &str) -> eyre::Result<Self> {
        let (action, target) = s
            .split_once('-')
            .ok_or(eyre::eyre!("missing action type or target"))?;

        let action = match action {
            "read" => Self::Read(Target::new(target)?),
            "write" => Self::Write(Target::new(target)?),
            "execute" => Self::Execute(Target::new(target)?),
            _ => return Err(eyre::eyre!("invalid action type {}", s)),
        };

        Ok(action)
    }
}

impl TryFrom<String> for Actionable {
    type Error = eyre::Report;

    fn try_from(s: String) -> eyre::Result<Self> {
        Self::try_from(s.as_str())
    }
}

impl fmt::Display for Actionable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Read(target) => format!("{}-{}", "read", target),
            Self::Write(target) => format!("{}-{}", "write", target),
            Self::Execute(target) => format!("{}-{}", "execute", target),
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub user_id: Uuid,
    pub action: Actionable,
    pub resource: Resource,
}

impl Permission {
    pub fn new(
        user_id: &str,
        action: &str,
        resource: &Resource,
    ) -> eyre::Result<Self, FieldValidationError> {
        Ok(Self {
            user_id: Uuid::try_from(user_id).map_err(|_| FieldValidationError {
                field: "user_id".to_string(),
                message: "invalid uuid provided for field `user_id`".to_string(),
            })?,
            action: Actionable::try_from(action.to_string()).map_err(|e| FieldValidationError {
                field: "action".to_string(),
                message: e.to_string(),
            })?,
            resource: resource.clone(),
        })
    }
}

impl fmt::Display for Permission {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.user_id, self.action, self.resource)
    }
}

impl TryFrom<&str> for Permission {
    type Error = eyre::Report;

    fn try_from(s: &str) -> eyre::Result<Self> {
        let segments = s.split(':').collect::<Vec<&str>>();
        if segments.len() != 4 {
            return Err(eyre::eyre!("invalid permission string format; correct format should be `user_id:action:resource_id:resource_kind`"));
        }
        let user_id = *segments
            .first()
            .ok_or(eyre::eyre!("missing `user_id` in permission string"))?;
        let action = *segments
            .get(1)
            .ok_or(eyre::eyre!("missing `action` in permission string"))?;
        let resource_id = *segments
            .get(2)
            .ok_or(eyre::eyre!("missing `resource_id` in permission string"))?;
        let resource_kind = *segments
            .get(3)
            .ok_or(eyre::eyre!("missing `resource_kind` in permission string"))?;

        Ok(Self {
            user_id: Uuid::try_from(user_id)?,
            action: Actionable::try_from(action)?,
            resource: Resource::try_from((resource_id, resource_kind))?,
        })
    }
}
