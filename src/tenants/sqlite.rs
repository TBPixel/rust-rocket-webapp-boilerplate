use crate::tenants::types;
use crate::types::uuid::Uuid;

use color_eyre::eyre;
use sqlx::SqliteExecutor;
use std::convert::TryFrom;

struct TenantRecord {
    id: String,
    name: String,
    created_at: chrono::NaiveDateTime,
}

pub async fn find_one<'e>(
    executor: impl SqliteExecutor<'e>,
    id: &str,
) -> eyre::Result<Option<types::Tenant>, sqlx::Error> {
    let tenant = match sqlx::query_as!(
        TenantRecord,
        "SELECT id, name, created_at FROM tenants WHERE id = ?",
        id
    )
    .fetch_one(executor)
    .await
    {
        Ok(u) => u,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Ok(None),
            e => return Err(e),
        },
    };

    Ok(Some(types::Tenant {
        id: Uuid::try_from(tenant.id).unwrap(),
        name: tenant.name,
        created_at: tenant.created_at,
    }))
}

pub async fn insert<'e>(
    executor: impl SqliteExecutor<'e>,
    tenant: &types::Tenant,
) -> eyre::Result<types::Tenant, sqlx::Error> {
    let id: String = tenant.id.clone().to_string();
    sqlx::query!(
        "
INSERT INTO tenants (id, name, created_at)
VALUES (?, ?, ?)
    ",
        id,
        tenant.name,
        tenant.created_at
    )
    .execute(executor)
    .await?;

    Ok(tenant.clone())
}

pub async fn delete<'e>(
    executor: impl SqliteExecutor<'e>,
    id: &str,
) -> eyre::Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM tenants WHERE id = ?", id)
        .execute(executor)
        .await?;

    Ok(())
}
