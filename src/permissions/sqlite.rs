use crate::permissions::types;

use color_eyre::eyre;
use sqlx::SqliteExecutor;

pub async fn insert<'e>(
    executor: impl SqliteExecutor<'e>,
    permission: &types::Permission,
) -> eyre::Result<types::Permission, sqlx::Error> {
    let user_id: String = permission.user_id.to_string();
    let action: String = permission.action.to_string();
    let resource_id = permission.resource.id();
    let resource_kind = permission.resource.kind().to_string();

    sqlx::query!(
        "
INSERT INTO permissions (user_id, action, resource_id, resource_kind)
VALUES (?, ?, ?, ?)
    ",
        user_id,
        action,
        resource_id,
        resource_kind
    )
    .execute(executor)
    .await?;

    Ok(permission.clone())
}

pub async fn delete<'e>(
    executor: impl SqliteExecutor<'e>,
    permission: &types::Permission,
) -> eyre::Result<(), sqlx::Error> {
    let user_id: String = permission.user_id.to_string();
    let action: String = permission.action.to_string();
    let resource_id = permission.resource.id();
    let resource_kind = permission.resource.kind().to_string();

    sqlx::query!(
        "DELETE FROM permissions
    WHERE user_id = ?
    AND action = ?
    AND resource_id = ?
    AND resource_kind = ?",
        user_id,
        action,
        resource_id,
        resource_kind
    )
    .execute(executor)
    .await?;

    Ok(())
}

pub async fn has_permission_to<'e>(
    executor: impl SqliteExecutor<'e>,
    permission: &types::Permission,
) -> eyre::Result<bool, sqlx::Error> {
    let user_id: String = permission.user_id.to_string();
    let action: String = permission.action.to_string();
    let resource_id = permission.resource.id();
    let resource_kind = permission.resource.kind().to_string();

    let exists = sqlx::query!(
        "SELECT COUNT(1) as count
            FROM permissions
            WHERE user_id = ?
            AND action = ?
            AND resource_id = ?
            AND resource_kind = ?",
        user_id,
        action,
        resource_id,
        resource_kind
    )
    .fetch_optional(executor)
    .await?
    .is_some();

    Ok(exists)
}
