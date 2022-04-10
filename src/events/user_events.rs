use anyhow::{ anyhow, Result };
use async_trait::async_trait;
use log::{ error, info };
use tokio::sync::{ mpsc::Sender, oneshot };

use crate::state::Message;
use super::UserEvent;

pub struct AppQuit;
#[async_trait]
impl UserEvent for AppQuit {
    fn get_name(&self) -> String {
        "AppQuit".to_string()
    }
    async fn trigger(&self, mpsc_send: Sender<Message>) -> Result<()> {
        info!("AppQuit triggered.");
        let (tx, rx) = oneshot::channel::<bool>();
        if let Err(err) = mpsc_send.send(Message::AppQuit(tx)).await { 
            error!("Error sending message AppQuit: {}", err);
        }
        if let Ok(true) = rx.await {
            Ok(())
        } else {
            Err(anyhow!("AppQuit ACK not received."))
        }
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
