use crate::permissions;
use crate::profiles;
use crate::tenants;
use crate::users;

use bus::BusReader;
use color_eyre::eyre;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum AppEvent {
    User(users::events::UserEvent),
    Profile(profiles::events::ProfileEvent),
    Permission(permissions::events::PermissionEvent),
    Tenant(tenants::events::TenantEvent),
}

pub trait EventHandler: Send + Sync {
    fn handle(&self, rx: Arc<tokio::sync::Mutex<BusReader<AppEvent>>>) -> eyre::Result<()>;
}
