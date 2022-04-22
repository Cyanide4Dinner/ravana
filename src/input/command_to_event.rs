use anyhow::Result;
use log::{ debug, error, warn };
use tokio::sync::{ mpsc::Sender, oneshot };

use crate::{
    def::commands::*,
    events::user_events::*,
    tui::App
};

// Log success on execution of a command or log error otherwise.
macro_rules! exec_err_handle {
    ($a: expr, $b: expr) => {
        if let Err(e) = $a {
            error!("Failed to execute command {}: {}", $b, e);
        } else {
            debug!("Successfully executed command: {}", $b);
        }
    }
}

// TODO: Write test to check if all commands are parsed and each event is triggered properly.

// -----------------------------------------------------------------------------------------------------------
// * Parse a command.
// * Trigger corresponding events.
// -----------------------------------------------------------------------------------------------------------
pub fn exec_cmd(app: &mut App,
                      cmd: &str) -> Result<()> {
    debug!("Executing command {}", cmd);
    let args: Vec<&str> = cmd.split(" ").collect();

    match args[0] {
        APP_QUIT => { 
            debug!("APP_QUIT");
            // if let Some(os_send) = opt_os_send {
            //     exec_err_handle!(e_app_quit(mpsc_send, os_send).await, APP_QUIT);
            // } else {
            //     error!("Oneshot sender needed, provided None.");
            // }
        },
        _ => { 

        }
    }
    Ok(())
}
