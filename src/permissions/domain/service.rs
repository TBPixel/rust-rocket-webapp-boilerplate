use std::convert::TryFrom;

use crate::permissions;
use crate::permissions::types;
use crate::types::sqlite;
use crate::types::validation::FieldValidationError;

use color_eyre::eyre;
use sqlx::SqlitePool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HasPermissionError {
    #[error("invalid input")]
    InvalidInput(FieldValidationError),

    #[error("unexpected error from sqlx")]
    Sqlx(#[from] sqlx::Error),
}

pub async fn has_permission_to(
    pool: &SqlitePool,
    user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> eyre::Result<bool, HasPermissionError> {
    let resource = types::Resource::try_from((resource_id, resource_kind)).map_err(|e| {
        HasPermissionError::InvalidInput(FieldValidationError {
            field: "resource".to_string(),
            message: e.to_string(),
        })
    })?;

    let permission = types::Permission::new(user_id, action, &resource)
        .map_err(HasPermissionError::InvalidInput)?;

    match permissions::sqlite::has_permission_to(pool, &permission).await {
        Ok(has_permission) => Ok(has_permission),
        Err(err) => Err(HasPermissionError::Sqlx(err)),
    }
}

#[derive(Error, Debug)]
pub enum GrantPermissionError {
    #[error("unauthorized to grant permission")]
    Unauthorized,

    #[error("failed to check permission of current user")]
    AccessCheckFailed(HasPermissionError),

    #[error("failed to create permission for new user")]
    CreateFailed(CreatePermissionError),
}

pub async fn grant_permission(
    pool: &SqlitePool,
    requesting_user_id: &str,
    receiving_user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> eyre::Result<types::Permission, GrantPermissionError> {
    match has_permission_to(pool, requesting_user_id, action, resource_id, resource_kind).await {
        Ok(has_permission) => {
            if !has_permission {
                return Err(GrantPermissionError::Unauthorized);
            }
        }
        Err(err) => return Err(GrantPermissionError::AccessCheckFailed(err)),
    }

    match create_permission(pool, receiving_user_id, action, resource_id, resource_kind).await {
        Ok(p) => Ok(p),
        Err(e) => Err(GrantPermissionError::CreateFailed(e)),
    }
}

#[derive(Error, Debug)]
pub enum RevokePermissionError {
    #[error("unauthorized to revoke permission")]
    Unauthorized,

    #[error("failed to check permission of requesting user")]
    AccessCheckFailed(HasPermissionError),

    #[error("failed to create permission for recipient user")]
    DeleteFailed(HasPermissionError),
}

pub async fn revoke_permission(
    pool: &SqlitePool,
    requesting_user_id: &str,
    receiving_user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> eyre::Result<(), RevokePermissionError> {
    match has_permission_to(pool, requesting_user_id, action, resource_id, resource_kind).await {
        Ok(has_permission) => {
            if !has_permission {
                return Err(RevokePermissionError::Unauthorized);
            }
        }
        Err(err) => return Err(RevokePermissionError::AccessCheckFailed(err)),
    }

    if let Err(err) =
        delete_permission(pool, receiving_user_id, action, resource_id, resource_kind).await
    {
        return Err(RevokePermissionError::DeleteFailed(err));
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum CreatePermissionError {
    #[error("invalid input")]
    InvalidInput(FieldValidationError),

    #[error("unexpected error from sqlx")]
    Sqlx(#[from] sqlx::Error),
}

async fn create_permission(
    pool: &SqlitePool,
    user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> eyre::Result<types::Permission, CreatePermissionError> {
    let resource = types::Resource::try_from((resource_id, resource_kind)).map_err(|e| {
        CreatePermissionError::InvalidInput(FieldValidationError {
            field: "resource".to_string(),
            message: e.to_string(),
        })
    })?;
    let permission = types::Permission::new(user_id, action, &resource)
        .map_err(CreatePermissionError::InvalidInput)?;

    let mut tx = pool
        .begin()
        .await
        .map_err(CreatePermissionError::Sqlx)?;
    if let Err(err) = permissions::sqlite::insert(&mut tx, &permission).await {
        match err {
            sqlx::Error::Database(e) => {
                if let Some(code) = e.code() {
                    // we can ignore a unique constraint violation since it's essentially
                    // equivalent to a noop
                    if let sqlite::ErrorCode::UniqueConstraintViolation =
                        sqlite::ErrorCode::from(code)
                    {
                        return Ok(permission);
                    }
                }
            }
            err => return Err(CreatePermissionError::Sqlx(err)),
        }
    }

    tx.commit()
        .await
        .map_err(CreatePermissionError::Sqlx)?;
    Ok(permission)
}

async fn delete_permission(
    pool: &SqlitePool,
    user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> eyre::Result<(), HasPermissionError> {
    let resource = types::Resource::try_from((resource_id, resource_kind)).map_err(|e| {
        HasPermissionError::InvalidInput(FieldValidationError {
            field: "resource".to_string(),
            message: e.to_string(),
        })
    })?;
    let permission = types::Permission::new(user_id, action, &resource)
        .map_err(HasPermissionError::InvalidInput)?;

    let mut tx = pool
        .begin()
        .await
        .map_err(HasPermissionError::Sqlx)?;
    permissions::sqlite::delete(&mut tx, &permission)
        .await
        .map_err(HasPermissionError::Sqlx)?;

    tx.commit().await.map_err(HasPermissionError::Sqlx)?;
    Ok(())
}
