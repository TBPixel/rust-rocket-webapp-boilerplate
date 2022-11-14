use crate::types::uuid::Uuid;
use crate::users::types;

use color_eyre::eyre;
use sqlx::SqliteExecutor;
use std::convert::TryFrom;

struct UserRecord {
    id: String,
    auth_id: String,
    created_at: chrono::NaiveDateTime,
}

pub async fn find_one<'e>(
    executor: impl SqliteExecutor<'e>,
    id: &str,
) -> eyre::Result<Option<types::User>, sqlx::Error> {
    let user = match sqlx::query_as!(
        UserRecord,
        "SELECT id, auth_id, created_at FROM users WHERE id = ?",
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

    Ok(Some(types::User {
        id: Uuid::try_from(user.id).unwrap(),
        auth_id: user.auth_id,
        created_at: user.created_at,
    }))
}

pub async fn insert<'e>(
    executor: impl SqliteExecutor<'e>,
    user: &types::User,
) -> eyre::Result<types::User, sqlx::Error> {
    let id: String = user.id.clone().to_string();
    sqlx::query!(
        "
INSERT INTO users (id, auth_id, created_at)
VALUES (?, ?, ?)
    ",
        id,
        user.auth_id,
        user.created_at
    )
    .execute(executor)
    .await?;

    Ok(user.clone())
}

pub async fn delete<'e>(
    executor: impl SqliteExecutor<'e>,
    id: &str,
) -> eyre::Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(executor)
        .await?;

    Ok(())
}
