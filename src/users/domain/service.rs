use std::convert::TryInto;

use crate::permissions;
use crate::profiles;
use crate::types::{uuid::Uuid, validation::FieldValidationError};
use crate::users::domain::events;
use crate::users::types;
use crate::{events::AppEvent, users};

use bus::Bus;
use color_eyre::eyre;
use serde::Deserialize;
use sqlx::SqlitePool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FindUserError {
    #[error("permission denied")]
    PermissionDenied,

    #[error("id `{0}` does not exist")]
    NotFound(String),

    #[error("unexpected error from sqlx")]
    Sqlx(#[from] sqlx::Error),
}

pub async fn find_user(pool: &SqlitePool, id: &str) -> eyre::Result<types::User, FindUserError> {
    let user = match users::sqlite::find_one(pool, id).await {
        Ok(user) => match user {
            Some(user) => user,
            None => return Err(FindUserError::NotFound(id.to_string())),
        },
        Err(err) => return Err(FindUserError::Sqlx(err)),
    };

    Ok(user)
}

#[derive(Error, Debug)]
pub enum CreateUserError {
    #[error("invalid input")]
    InvalidInput(FieldValidationError),

    #[error("unexpected error from sqlx")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub tenant_id: Uuid,
}

pub async fn create_user(
    pool: &SqlitePool,
    bus: &tokio::sync::Mutex<Bus<AppEvent>>,
    payload: CreateUserRequest,
) -> eyre::Result<types::User, CreateUserError> {
    let mut tx = pool.begin().await.map_err(CreateUserError::Sqlx)?;

    // mock the auth id;
    // in production this would be received from an external oauth service
    let mock_auth_id = uuid::Uuid::new_v4().to_string();
    let user = types::User::new(&mock_auth_id);

    if let Err(err) = users::sqlite::insert(&mut tx, &user).await {
        return Err(CreateUserError::Sqlx(err));
    }

    profiles::sqlite::insert(
        &mut tx,
        &profiles::CreateProfile {
            user_id: user.id.clone(),
            email: payload.email.try_into().map_err(|e: eyre::Report| {
                CreateUserError::InvalidInput(FieldValidationError {
                    field: "email".to_string(),
                    message: e.to_string(),
                })
            })?,
        },
    )
    .await
    .map_err(CreateUserError::Sqlx)?;

    let grants = vec![
        permissions::types::Permission {
            user_id: user.id.clone(),
            action: permissions::types::Actionable::Read(
                types::User::kind()
            ),
            resource: permissions::types::Resource::User(user.id.to_string()),
        },
        permissions::types::Permission {
            user_id: user.id.clone(),
            action: permissions::types::Actionable::Write(
                types::User::kind()
            ),
            resource: permissions::types::Resource::User(user.id.to_string()),
        },
    ];
    for grant in grants {
        permissions::sqlite::insert(&mut tx, &grant)
            .await
            .map_err(CreateUserError::Sqlx)?;
    }

    tx.commit().await.map_err(CreateUserError::Sqlx)?;

    bus.lock()
        .await
        .broadcast(AppEvent::User(events::UserEvent::Created(user.clone())));

    Ok(user)
}

pub async fn delete_user(
    pool: &SqlitePool,
    bus: &tokio::sync::Mutex<Bus<AppEvent>>,
    id: &str,
) -> eyre::Result<(), FindUserError> {
    match permissions::has_permission_to(pool, id, "write-user", id, &types::User::kind().to_string()).await {
        Ok(can) => {
            if !can {
                return Err(FindUserError::PermissionDenied);
            }
        }
        Err(err) => match err {
            permissions::HasPermissionError::Sqlx(err) => return Err(FindUserError::Sqlx(err)),
            _ => todo!(),
        },
    }

    let mut tx = pool.begin().await.map_err(FindUserError::Sqlx)?;
    users::sqlite::delete(&mut tx, id)
        .await
        .map_err(FindUserError::Sqlx)?;

    tx.commit().await.map_err(FindUserError::Sqlx)?;

    bus.lock()
        .await
        .broadcast(AppEvent::User(events::UserEvent::Deleted(id.to_string())));

    Ok(())
}
