use anyhow::{ anyhow, Result };
use libnotcurses_sys::*;
use log::{ debug, error, info };
use std::sync::{ Mutex, Arc };
use tokio::sync::mpsc;

use crate::input::{ listener::init as listener_init };
use crate::jobs::config::load_config;
use crate::state::{ manager::init as manager_init, Message };
use crate::tui::val_tui_prefs_des;

// -----------------------------------------------------------------------------------------------------------
// Initialize application -
// * Load config.
// * Create Nc instance.
// * Create channel for input -> manager communication.
// * Spawn input listener and manager (TUI) jobs.
// -----------------------------------------------------------------------------------------------------------
pub async fn init() -> Result<()> {
    let config = load_config().await;

    // Validate config
    // TODO: Validate key-bindings
    {
        debug!("Validating config.");
        if !val_tui_prefs_des(&config.tui) { 
            error!("Invalid TUI format in config.");
            return Err(anyhow!("Invalid TUI format in config."));
        } 
    }

    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let (mpsc_tx, mpsc_rx) = mpsc::channel::<Message>(32);

    tokio::spawn(manager_init(Arc::clone(&nc), config.tui, mpsc_rx, mpsc_tx.clone()));
    listener_init(Arc::clone(&nc), config.key_bindings, mpsc_tx.clone()).await.unwrap();

    Ok(()) 
}
