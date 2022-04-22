use anyhow::{ anyhow, Result };
use log::{ debug, error };
use tokio::sync::{ mpsc::Sender, oneshot };

use crate::tui::App;

// -----------------------------------------------------------------------------------------------------------
// * Send message to quit TUI.
// -----------------------------------------------------------------------------------------------------------
pub fn e_app_quit(app: &mut App) -> Result<()> {
        debug!("AppQuit triggered.");
        Ok(())
}
