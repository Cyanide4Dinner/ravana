use anyhow::Result;
use log::{ debug, info, warn };
use libnotcurses_sys::{
    Nc
};
use std::sync::{ Arc, Mutex };
use std::{ thread, time };
use tokio::sync::mpsc::Receiver;

use crate::{
    jobs::TuiPrefsDes,
    state::Message,
    tui::{ App, init_tui }
};

// Initialize state manager.
pub async fn init(
        nc: Arc<Mutex<&mut Nc>>, 
        tui_prefs_des: TuiPrefsDes,
        mpsc_recv: Receiver<Message>) -> Result<()> {
    thread::sleep(time::Duration::new(5,0));
    manage(nc, tui_prefs_des, mpsc_recv).await?;
    Ok(())
}

// TODO: Figure out need for InitTUI.

// -----------------------------------------------------------------------------------------------------------
// * Listen for event message and manipulate state of app or trigger new events.
// -----------------------------------------------------------------------------------------------------------
pub async fn manage(
        nc: Arc<Mutex<&mut Nc>>,
        tui_prefs_des: TuiPrefsDes,
        mut mpsc_recv: Receiver<Message>) -> Result<()> {
    let mut app: App = init_tui(nc.clone(), &tui_prefs_des)?;
    loop {
        if let Some(ms) = mpsc_recv.recv().await {
            match ms {
                Message::CmdInput(ncin) => {
                    // info!("CMD input!");
                    app.input_cmd_plt(ncin)?;
                },
                Message::InitTUI => {
                    debug!("Message recieved: TUI init");
                    // app = init_tui(nc.clone(), &tui_prefs_des)?;
                },
                Message::AppQuit(tx) => {
                    drop(app);
                    tx.send(true);
                    debug!("Message recieved: App quit");
                    return Ok(());
                },
            } 
        }
        else {
            warn!("None type message recieved from recv().");
        }
    }
}
