use anyhow::{ Result };
use libnotcurses_sys::*;
use log::info;
use std::sync::{ Mutex, Arc };
use tokio::sync::oneshot;

use crate::input::listener::init as listener_init;
use crate::jobs::{ config::load_config };
use crate::state::manager::init as manager_init;

//TODO: Close nc.
pub async fn init() -> Result<()> {
    info!("Loading config.");
    let config = Arc::new(load_config().await);
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));
    
    // tokio::spawn(listen_init(Arc::clone(&nc), config.clone()));
    // listen_init(Arc::clone(&nc), Arc::clone(&config)).await.unwrap();
    Ok(()) 
}

pub fn test_tui() -> NcResult<()> {
    //Testing
    let nc = unsafe { Nc::new()? };
    let splane = unsafe { nc.stdplane() };
    splane.set_scrolling(true);

    // tokio::spawn(manager_init(Arc::clone(&nc)));

    // tokio::spawn(listen_init(Arc::clone(&nc), Arc::clone(&config)));
    listener_init(Arc::clone(&nc), Arc::clone(&config)).await.unwrap();
    Ok(()) 
}
