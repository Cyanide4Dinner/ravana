use anyhow::Result;
use log::{ info, warn };
use libnotcurses_sys::{
    Nc
};
use std::sync::{ Arc, Mutex };
use std::{ thread, time };
use tokio::sync::mpsc::Receiver;

use crate::state::Message;

pub async fn init(nc: Arc<Mutex<&mut Nc>>, mpsc_recv: Receiver<Message>) -> Result<()> {
    thread::sleep(time::Duration::new(5,0));

    // let mut nc_lock = nc.lock().unwrap();
    // let stdp = unsafe { nc_lock.stdplane() };
    // stdp.erase();
    // stdp.putstrln("Hello world 2.")?;
    // nc_lock.render()?;
    // drop(nc_lock);

    manage(nc, mpsc_recv).await?;
    Ok(())
}

pub async fn manage(nc: Arc<Mutex<&mut Nc>>, mut mpsc_recv: Receiver<Message>) -> Result<()> {
    loop {
        if let Some(ms) = mpsc_recv.recv().await {
            match ms {
                InitTUI => {
                    info!("TUI init message recieved.");
                    // let mut nc_lock = nc.lock().unwrap();
                    // let stdp = unsafe { nc_lock.stdplane() };
                    // stdp.erase();
                    // stdp.putstrln("Hello world 2.")?;
                    // nc_lock.render()?;
                    // drop(nc_lock);
                }
            } 
        }
        else {
            warn!("None type message recieved from recv().");
        }
    }
}
