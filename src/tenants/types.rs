use crate::types::uuid::Uuid;
use crate::permissions;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub created_at: chrono::NaiveDateTime,
}

impl Tenant {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new(),
            name: name.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn kind() -> permissions::types::Target {
        permissions::types::Target("tenant".to_string())
    }
}
