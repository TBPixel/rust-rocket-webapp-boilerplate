use crate::auth::service;
use crate::events::AppEvent;
use crate::users;

use bus::Bus;
use color_eyre::eyre;
use rocket::{http::Status, route::Route, serde::json::Json};
use sqlx::SqlitePool;

#[allow(clippy::no_effect_underscore_binding)]
pub fn routes() -> Vec<Route> {
    routes![sign_in_route, sign_up_route]
}

#[post("/", data = "<payload>")]
async fn sign_in_route(
    pool: &rocket::State<SqlitePool>,
    payload: Json<service::AuthRequest>,
) -> eyre::Result<Json<users::types::User>, Status> {
    let profile = service::sign_in(pool.inner(), &payload.into_inner())
        .await
        .unwrap();
    let user = users::find_user(pool, &profile.user_id.to_string())
        .await
        .unwrap();

    Ok(Json(user))
}

#[post("/sign-up", data = "<payload>")]
async fn sign_up_route(
    pool: &rocket::State<SqlitePool>,
    bus: &rocket::State<tokio::sync::Mutex<Bus<AppEvent>>>,
    payload: Json<service::AuthRequest>,
) -> eyre::Result<Json<users::types::User>, Status> {
    let user = service::sign_up(pool, bus.inner(), &payload).await.unwrap();

    Ok(Json(user))
}
