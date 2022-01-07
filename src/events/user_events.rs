use anyhow::Result;
use async_trait::async_trait;

use super::UserEvent;

pub struct AppQuit;
#[async_trait]
impl UserEvent for AppQuit {
    async fn trigger(&self) -> Result<()> {
        Ok(())
    }
}

// TODO: Create better solution for debug event.
#[cfg(feature = "dev")]
pub struct TestHello;
#[async_trait]
#[cfg(feature = "dev")]
impl UserEvent for TestHello {
    async fn trigger(&self) -> Result<()> {
        println!("Hello <3.");
        Ok(())
    }
}
