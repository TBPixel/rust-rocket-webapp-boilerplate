use crate::profiles::{domain::service, types};
use crate::types::uuid::Uuid;

use color_eyre::eyre;
use sqlx::SqliteExecutor;
use std::convert::TryFrom;

struct ProfileRecord {
    user_id: String,
    email: String,
}

pub async fn find_one<'e>(
    executor: impl SqliteExecutor<'e>,
    email: &str,
) -> eyre::Result<Option<types::Profile>, sqlx::Error> {
    let profile = match sqlx::query_as!(
        ProfileRecord,
        "SELECT user_id, email FROM profiles WHERE email = ?",
        email
    )
    .fetch_one(executor)
    .await
    {
        Ok(p) => p,
        Err(e) => match e {
            sqlx::Error::RowNotFound => return Ok(None),
            e => return Err(e),
        },
    };

    Ok(Some(types::Profile {
        user_id: Uuid::try_from(profile.user_id).unwrap(),
        email: types::Email::new(&profile.email).unwrap(),
    }))
}

pub async fn insert<'e>(
    executor: impl SqliteExecutor<'e>,
    profile: &service::CreateProfile,
) -> eyre::Result<types::Profile, sqlx::Error> {
    let user_id: String = profile.user_id.to_string();
    let email: String = profile.email.to_string();
    sqlx::query!(
        "
INSERT INTO profiles (user_id, email)
VALUES (?, ?)
    ",
        user_id,
        email
    )
    .execute(executor)
    .await?;

    Ok(types::Profile {
        user_id: profile.user_id.clone(),
        email: profile.email.clone(),
    })
}
