use crate::events;

use bus::Bus;
use rocket::{fairing, fairing::Fairing, http, Build, Rocket};
use sqlx::SqlitePool;
use std::env;
use std::sync::Arc;
use uuid::Uuid;

pub struct EventProcessor {
    handlers: Vec<Arc<dyn events::EventHandler>>,
}

impl EventProcessor {
    pub fn new(handlers: Vec<Arc<dyn events::EventHandler>>) -> Self {
        Self { handlers }
    }
}

#[rocket::async_trait]
impl Fairing for EventProcessor {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "EventProcessor",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let size = env::var("EVENT_PROCESS_BUS_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse::<usize>()
            .unwrap_or_else(|e| panic!("{}", e));
        let mut bus = Bus::new(size);

        for handler in &self.handlers {
            handler
                .handle(Arc::new(tokio::sync::Mutex::new(bus.add_rx())))
                .unwrap_or_else(|e| panic!("{}", e.to_string()));
        }

        let bus: tokio::sync::Mutex<Bus<events::AppEvent>> = tokio::sync::Mutex::new(bus);
        Ok(rocket.manage::<tokio::sync::Mutex<Bus<events::AppEvent>>>(bus))
    }
}

pub struct SqliteDatabase;

#[rocket::async_trait]
impl Fairing for SqliteDatabase {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Database",
            kind: fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> fairing::Result {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| panic!("DATABASE_URL env var must be defined"));
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(4)
            .idle_timeout(std::time::Duration::from_secs(5))
            .connect(&database_url)
            .await
            .unwrap_or_else(|_| panic!("to connect to database @ {}", database_url));
        tracing::info!("connected with database @ {}", database_url);
        sqlx::migrate!()
            .run(&pool)
            .await
            .unwrap_or_else(|_| panic!("failed to run migrations"));
        tracing::info!("latest database migrations executed");
        sqlx::query!("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .unwrap_or_else(|_| panic!("failed to enable foreign key contraints on startup"));
        tracing::info!("foreign key constraints enabled");

        Ok(rocket.manage::<SqlitePool>(pool))
    }
}

pub const REQUEST_ID_HEADER: &str = "x-request-id";
pub struct RequestID;

#[rocket::async_trait]
impl Fairing for RequestID {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "RequestID",
            kind: fairing::Kind::Request | fairing::Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut rocket::Request<'_>, _: &mut rocket::Data<'_>) {
        match request.method() {
            http::Method::Options | http::Method::Connect | http::Method::Trace => return,
            _ => request.add_header(http::Header::new(
                REQUEST_ID_HEADER,
                Uuid::new_v4().to_string(),
            )),
        };
    }

    async fn on_response<'r>(
        &self,
        request: &'r rocket::Request<'_>,
        response: &mut rocket::Response<'r>,
    ) {
        if let Some(request_id) = request.headers().get_one(REQUEST_ID_HEADER) {
            response.set_header(http::Header::new(REQUEST_ID_HEADER, request_id));
        }
    }
}
