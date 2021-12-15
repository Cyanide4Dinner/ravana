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
