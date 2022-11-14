use crate::events;
use crate::users::types::User;

use color_eyre::eyre;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserEvent {
    Created(User),
    Deleted(String),
}

pub struct UsersEventHandler;

impl UsersEventHandler {
    pub fn new_handler() -> Arc<dyn events::EventHandler> {
        Arc::new(Self {})
    }
}

impl events::EventHandler for UsersEventHandler {
    fn handle(
        &self,
        rx: Arc<tokio::sync::Mutex<bus::BusReader<events::AppEvent>>>,
    ) -> eyre::Result<()> {
        tokio::spawn(async move {
            let mut rx = rx.lock().await;
            loop {
                if let Ok(event) = rx.recv() {
                    tracing::debug!("recv: {:?}", event);
                }
            }
        });

        Ok(())
    }
}
