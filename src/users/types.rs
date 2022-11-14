use crate::types::uuid::Uuid;
use crate::permissions;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub auth_id: String,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(auth_id: &str) -> Self {
        Self {
            id: Uuid::new(),
            auth_id: auth_id.to_string(),
            created_at: chrono::Utc::now().naive_utc(),
        }
    }

    pub fn kind() -> permissions::types::Target {
        permissions::types::Target("user".to_string())
    }
}
