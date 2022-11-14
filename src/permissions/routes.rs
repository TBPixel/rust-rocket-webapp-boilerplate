use crate::permissions::domain::service;

use rocket::{http::Status, route::Route};
use sqlx::SqlitePool;

#[allow(clippy::no_effect_underscore_binding)]
pub fn routes() -> Vec<Route> {
    routes![
        has_permission_to,
        grant_user_permission_route,
        revoke_user_permission_route
    ]
}

#[get("/<user_id>/<action>/<resource_id>/<resource_kind>")]
async fn has_permission_to(
    pool: &rocket::State<SqlitePool>,
    user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> Status {
    let has_permission =
        match service::has_permission_to(pool, user_id, action, resource_id, resource_kind).await {
            Ok(has_permission) => has_permission,
            Err(_) => todo!(),
        };

    if has_permission {
        Status::Ok
    } else {
        todo!()
    }
}

#[post("/<user_id>/<action>/<resource_id>/<resource_kind>")]
async fn grant_user_permission_route(
    pool: &rocket::State<SqlitePool>,
    user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> Status {
    let requesting_user_id = "mock-testing-id";
    match service::grant_permission(
        pool,
        requesting_user_id,
        user_id,
        action,
        resource_id,
        resource_kind,
    )
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => todo!(),
    }
}

#[delete("/<user_id>/<action>/<resource_id>/<resource_kind>")]
async fn revoke_user_permission_route(
    pool: &rocket::State<SqlitePool>,
    user_id: &str,
    action: &str,
    resource_id: &str,
    resource_kind: &str,
) -> Status {
    let requesting_user_id = "mock-testing-id";
    match service::revoke_permission(
        pool,
        requesting_user_id,
        user_id,
        action,
        resource_id,
        resource_kind,
    )
    .await
    {
        Ok(_) => Status::Ok,
        Err(_) => todo!(),
    }
}
