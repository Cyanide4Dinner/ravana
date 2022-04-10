use anyhow::{ anyhow, Result };
use async_trait::async_trait;
use tokio::sync::mpsc::Sender;

use crate::state::Message;
use super::user_events;

// Encapsulated event for key-bindings.
#[async_trait]
pub trait UserEvent: Send {
    fn get_name(&self) -> String;
    async fn trigger(&self, mpsc_send: Sender<Message>) -> Result<()>;
}

// Fetch dynamically dispatched UserEvent corresponding to "Field-name" in Config.toml
pub fn get_user_event(field_name: &str) -> Result<Box<dyn UserEvent>> {
    match field_name {
        "app_quit" => { Ok(Box::new(user_events::AppQuit)) },
        _ =>  { Err(anyhow!("Cannot find UserEvent corresponding to: {}", field_name)) },
    }
}
