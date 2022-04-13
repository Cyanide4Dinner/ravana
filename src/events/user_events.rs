use anyhow::{ anyhow, Result };
use async_trait::async_trait;
use log::{ debug, error, info };
use tokio::sync::{ mpsc::Sender, oneshot };

use crate::state::Message;
use super::UserEvent;

// -----------------------------------------------------------------------------------------------------------
// * Send message to quit TUI.
// -----------------------------------------------------------------------------------------------------------
pub struct AppQuit;

#[async_trait]
impl UserEvent for AppQuit {
    fn get_name(&self) -> String {
        "AppQuit".to_string()
    }
    async fn trigger(&self, mpsc_send: Sender<Message>) -> Result<()> {
        debug!("AppQuit triggered.");
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

pub async fn e_app_quit(mpsc_send: Sender<Message>) -> Result<()> {
        debug!("AppQuit triggered.");
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
