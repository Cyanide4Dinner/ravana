use anyhow::{ Result };
use libnotcurses_sys::*;
use log::info;
use std::sync::{ Mutex, Arc };
use tokio::sync::mpsc;

use crate::input::listener::init as listener_init;
use crate::jobs::{ config::load_config };
use crate::state::{ manager::init as manager_init, Message };

//TODO: Close nc.
pub async fn init() -> Result<()> {
    info!("Loading config.");
    let config = Arc::new(load_config().await);
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let (tx, rx) = mpsc::channel::<Message>(32);

    tokio::spawn(manager_init(Arc::clone(&nc), rx));
    listener_init(Arc::clone(&nc), Arc::clone(&config), tx.clone()).await.unwrap();

    Ok(()) 
}
