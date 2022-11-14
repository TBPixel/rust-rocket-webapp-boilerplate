use std::convert::TryInto;

use crate::events::AppEvent;
use crate::profiles;
use crate::users;

use bus::Bus;
use color_eyre::eyre;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub email: String,
    pub password: String,
    pub tenant_id: String,
}

pub async fn sign_in(
    pool: &SqlitePool,
    payload: &AuthRequest,
) -> eyre::Result<profiles::types::Profile> {
    let profile = profiles::find_profile(pool, &payload.email).await?;

    Ok(profile)
}

pub async fn sign_up(
    pool: &SqlitePool,
    bus: &tokio::sync::Mutex<Bus<AppEvent>>,
    payload: &AuthRequest,
) -> eyre::Result<users::types::User> {
    let tenant_id = payload.tenant_id.clone();
    let user = users::create_user(
        pool,
        bus,
        users::CreateUserRequest {
            email: payload.email.clone(),
            tenant_id: tenant_id.try_into().unwrap(),
        },
    )
    .await
    .unwrap();

    Ok(user)
}
