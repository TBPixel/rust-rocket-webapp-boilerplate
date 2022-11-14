use crate::{
    events::AppEvent,
    users::{domain::service, types},
};

use bus::Bus;
use color_eyre::eyre;
use rocket::{http::Status, route::Route, serde::json::Json};
use sqlx::SqlitePool;

#[allow(clippy::no_effect_underscore_binding)]
pub fn routes() -> Vec<Route> {
    routes![find_user_route, delete_user_route]
}

#[get("/<id>")]
async fn find_user_route(
    pool: &rocket::State<SqlitePool>,
    id: &str,
) -> eyre::Result<Json<types::User>, Status> {
    match service::find_user(pool.inner(), id).await {
        Ok(user) => Ok(Json(user)),
        Err(err) => match err {
            service::FindUserError::NotFound(_) => Err(Status::NotFound),
            _ => todo!(),
        },
    }
}

#[delete("/<id>")]
async fn delete_user_route(
    pool: &rocket::State<SqlitePool>,
    bus: &rocket::State<tokio::sync::Mutex<Bus<AppEvent>>>,
    id: &str,
) -> Status {
    if service::delete_user(pool.inner(), bus.inner(), id)
        .await
        .is_err()
    {
        todo!();
    }

    Status::Ok
}
