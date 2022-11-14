use crate::types::uuid::Uuid;
use crate::{profiles, profiles::types};

use color_eyre::eyre;
use sqlx::sqlite::SqlitePool;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CreateProfileError {
    #[error("unexpected error from sqlx")]
    Sqlx(#[from] sqlx::Error),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateProfile {
    pub user_id: Uuid,
    pub email: types::Email,
}

pub async fn create_profile(
    pool: &SqlitePool,
    profile: &CreateProfile,
) -> eyre::Result<types::Profile, CreateProfileError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(CreateProfileError::Sqlx)?;
    let profile = match profiles::sqlite::insert(&mut tx, profile).await {
        Ok(profile) => profile,
        Err(err) => return Err(CreateProfileError::Sqlx(err)),
    };

    tx.commit().await.map_err(CreateProfileError::Sqlx)?;
    Ok(profile)
}

#[derive(Error, Debug)]
pub enum FindProfileError {
    #[error("profile was not found")]
    NotFound(String),

    #[error("unexpected error from sqlx")]
    Sqlx(#[from] sqlx::Error),
}

pub async fn find_profile(
    pool: &SqlitePool,
    email: &str,
) -> eyre::Result<types::Profile, FindProfileError> {
    let profile = match profiles::sqlite::find_one(pool, email).await {
        Ok(profile) => match profile {
            Some(profile) => profile,
            None => return Err(FindProfileError::NotFound(email.to_string())),
        },
        Err(err) => return Err(FindProfileError::Sqlx(err)),
    };

    Ok(profile)
}
