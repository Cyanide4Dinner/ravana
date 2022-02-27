use anyhow::{ anyhow, Result };
use libnotcurses_sys::*;
use log::{ error, info, warn };
use std::sync::{ Mutex, Arc };
use tokio::sync::mpsc;

use crate::input::listener::init as listener_init;
use crate::jobs::{ config::load_config };
use crate::state::{ manager::init as manager_init, Message };
use crate::tui::val_tui_prefs_des;

//TODO: Close nc.
pub async fn init() -> Result<()> {
    info!("Loading config.");
    let config = load_config().await;

    // Validate config
    // TODO: Validate key-bindings
    {
        info!("Validating config.");
        if !val_tui_prefs_des(&config.tui) { 
            error!("Invalid TUI format in config.");
            anyhow!("Invalid TUI format in config.")
        } 
    }

    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let (tx, rx) = mpsc::channel::<Message>(32);

    tokio::spawn(manager_init(Arc::clone(&nc), config.tui, rx));
    listener_init(Arc::clone(&nc), config.key_bindings, tx.clone()).await.unwrap();

    Ok(()) 
}
