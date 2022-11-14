use crate::events::AppEvent;

use bus::Bus;
use color_eyre::eyre;
use rocket::{http::Status, route::Route, serde::json::Json};
use sqlx::SqlitePool;

#[allow(clippy::no_effect_underscore_binding)]
pub fn routes() -> Vec<Route> {
    routes![]
}
