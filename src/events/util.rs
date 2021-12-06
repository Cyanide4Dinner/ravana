use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EventError {
    #[error("Failed to handle event: {0}")]
    EventHandleFail(String),
    #[error("Event unfit to be triggered in current time and context: {0}")]
    BadEventTrigger(String)
}

// Encapsulated event for key-bindings.
#[async_trait]
pub trait UserEvent {
    async fn trigger(&self) -> Result<(), EventError>;
}
