use anyhow::Result;
use log::{ info, warn };
use libnotcurses_sys::{
    Nc
};
use std::sync::{ Arc, Mutex };
use std::{ thread, time };
use tokio::sync::mpsc::Receiver;

use crate::{
    state::Message,
    tui::{ App, init_tui }
};

pub async fn init(nc: Arc<Mutex<&mut Nc>>, mpsc_recv: Receiver<Message>) -> Result<()> {
    thread::sleep(time::Duration::new(5,0));
    manage(nc, mpsc_recv).await?;
    Ok(())
}

pub async fn manage(nc: Arc<Mutex<&mut Nc>>, mut mpsc_recv: Receiver<Message>) -> Result<()> {
    let mut app: App;
    loop {
        if let Some(ms) = mpsc_recv.recv().await {
            match ms {
                InitTUI => {
                    info!("TUI init message recieved.");
                    app = init_tui(nc.clone())?;
                }
            } 
        }
        else {
            warn!("None type message recieved from recv().");
        }
    }
}
