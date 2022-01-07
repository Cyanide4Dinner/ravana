use anyhow::{ anyhow, Result };
use log::error;
use std::{ thread::sleep, time::Duration };
use tokio::sync::mpsc::Sender;

use crate::state::Message;

// Sent by input loop to check readiness of TUI.
pub async fn init_tui(mpsc_send: Sender<Message>) -> Result<()> {
    let mut count = 0;
    loop {
        if let Ok(()) = mpsc_send.send(Message::InitTUI).await { break; }
        error!("Failed to send message: init_tui.");
        sleep(Duration::new(2,0));
        count = count + 1;
        if count > 5 { break; }
    }
    if count > 5 { return Err(anyhow!("Failed to send message: init_tui, tried 5 times.")); }
    Ok(())
}
