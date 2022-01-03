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
    
    // let mut nc_lock = nc.lock().unwrap();
    // let stdp = unsafe { nc_lock.stdplane() };
    // stdp.putstrln("Hello world.")?;
    // nc_lock.render()?;
    // drop(nc_lock);

    // tokio::spawn(manager_init(Arc::clone(&nc)));

    // tokio::spawn(listen_init(Arc::clone(&nc), Arc::clone(&config)));
    listener_init(Arc::clone(&nc), Arc::clone(&config)).await.unwrap();
    Ok(()) 
}
