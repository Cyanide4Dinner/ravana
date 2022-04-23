use anyhow::Result;

use crate::{
    def::commands::*,
    tui::App
};


// TODO: Write test to check if all commands are parsed and each event is triggered properly.

// -----------------------------------------------------------------------------------------------------------
// * Parse a command.
// * Trigger corresponding events.
// -----------------------------------------------------------------------------------------------------------
pub fn exec_cmd(app: &mut App,
                      cmd: &str) -> Result<()> {
    let args: Vec<&str> = cmd.split(" ").collect();

    match args[0] {
        APP_QUIT => { 
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
