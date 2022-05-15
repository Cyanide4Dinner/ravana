use anyhow::Result;

use crate::{
    def::commands::*,
    tui::{ App, AppRes }
};


// TODO: Write test to check if all commands are parsed and each event is triggered properly.

// -----------------------------------------------------------------------------------------------------------
// * Parse a command.
// * Trigger corresponding events.
// -----------------------------------------------------------------------------------------------------------
pub fn exec_cmd(app: &mut App,
                      cmd: &str) -> Result<Option<AppRes>> {
    let args: Vec<&str> = cmd.split(" ").collect();

    match args[0] {
        APP_QUIT => { 
            // if let Some(os_send) = opt_os_send {
            //     exec_err_handle!(e_app_quit(mpsc_send, os_send).await, APP_QUIT);
            // } else {
            //     error!("Oneshot sender needed, provided None.");
            // }
            Ok(Some(AppRes::AppQuit))
        },
        SCROLL_DOWN => {
            app.scroll_down();
            app.render()?;
            Ok(None)
        },
        SCROLL_UP => {
            app.scroll_up();
            app.render()?;
            Ok(None)
        },
        SWITCH_PAGE => {
            app.set_foc_page(args[1].parse::<usize>()?);
            app.render()?;
            Ok(None)
        },
        NEXT_PAGE => {
            app.switch_next_page();
            app.render()?;
            Ok(None)
        },
        PREV_PAGE => {
            app.switch_prev_page();
            app.render()?;
            Ok(None)
        }
        _ => { 
            Ok(None)
        }
    }
}
