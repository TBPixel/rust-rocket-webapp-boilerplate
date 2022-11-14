#![forbid(unsafe_code)]
#![warn(
    clippy::pedantic,
    clippy::nursery,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::todo,
    clippy::unimplemented
)]
#![allow(clippy::use_self)] // disabling use_self lints due to a bug where proc-macro's (such as serde::Serialize) can trigger it to hinted on type definitions

#[macro_use]
extern crate rocket;

mod auth;
mod events;
mod fairings;
mod permissions;
mod profiles;
mod tenants;
mod types;
mod users;

use color_eyre::eyre;
use rocket::http::Status;
use rocket::serde::json::Json;
use serde::Serialize;

#[rocket::main]
async fn main() -> eyre::Result<()> {
    // error tracing
    color_eyre::install()?;

    // load .env file; choose not to handle errors as .env file is only a convenience
    dotenvy::dotenv().ok();

    // install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // web framework
    let _rocket = rocket::build()
        .mount("/api", routes())
        .mount("/api/auth", auth::routes())
        .mount("/api/permissions", permissions::routes())
        .mount("/api/tenants", tenants::routes())
        .mount("/api/users", users::routes())
        .attach(fairings::RequestID)
        .attach(fairings::SqliteDatabase)
        .attach(fairings::EventProcessor::new(vec![
            permissions::events::PermissionsEventHandler::new_handler(),
            profiles::events::ProfilesEventHandler::new_handler(),
            tenants::events::TenantsEventHandler::new_handler(),
            users::events::UsersEventHandler::new_handler(),
        ]))
        .launch()
        .await?;

    Ok(())
}

#[allow(clippy::no_effect_underscore_binding)]
fn routes() -> Vec<rocket::route::Route> {
    routes![index, health]
}

#[get("/")]
const fn index() -> Status {
    Status::Ok
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    pub datetime: chrono::NaiveDateTime,
}
#[get("/health")]
fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        datetime: chrono::Utc::now().naive_local(),
    })
}
