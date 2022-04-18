use anyhow::{ anyhow, Result };
use async_trait::async_trait;
use log::{ debug, error, info };
use tokio::sync::{ mpsc::Sender, oneshot };

use crate::state::Message;
use crate::input::input_message::InputMessage;

// -----------------------------------------------------------------------------------------------------------
// * Send message to quit TUI.
// -----------------------------------------------------------------------------------------------------------
pub async fn e_app_quit(mpsc_send: Sender<Message>, oneshot_send: oneshot::Sender::<InputMessage>) -> Result<()> {
        debug!("AppQuit triggered.");
        if let Err(err) = mpsc_send.send(Message::AppQuit(oneshot_send)).await { 
            error!("Error sending message AppQuit: {}", err);
            return Err(anyhow!("Error sending message AppQuit: {}", err));
        }
        Ok(())
}
