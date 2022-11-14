use crate::events;
use crate::profiles::types::Profile;

use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ProfileEvent {
    Created(Profile),
    Deleted(String),
}

pub struct ProfilesEventHandler;

impl ProfilesEventHandler {
    pub fn new_handler() -> Arc<dyn events::EventHandler> {
        Arc::new(Self {})
    }
}

impl events::EventHandler for ProfilesEventHandler {
    fn handle(
        &self,
        rx: Arc<tokio::sync::Mutex<bus::BusReader<events::AppEvent>>>,
    ) -> color_eyre::eyre::Result<()> {
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
