use anyhow::{ Result };
use libnotcurses_sys::*;
use log::info;
use std::sync::{ Mutex, Arc };

use crate::input::listener::init as listen_init;
use crate::jobs::{ config::load_config };

//TODO: Close nc.
pub async fn init() -> Result<()> {
    info!("Loading config.");
    let config = Arc::new(load_config().await);
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));
    
    // tokio::spawn(listen_init(Arc::clone(&nc), Arc::clone(&config)));
    listen_init(Arc::clone(&nc), Arc::clone(&config)).await.unwrap();
    Ok(()) 
}
