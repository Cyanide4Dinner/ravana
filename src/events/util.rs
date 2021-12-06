use anyhow::Result;
use async_trait::async_trait;

// Encapsulated event for key-bindings.
#[async_trait]
pub trait UserEvent {
    async fn trigger(&self) -> Result<()>;
}
