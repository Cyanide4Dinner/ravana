use anyhow::{ Context, Result };
use libnotcurses_sys::Nc;
use log::{ debug, error };
use std::sync::{ Arc, Mutex };

use crate::{ 
        jobs::TuiPrefsDes, 
        tools::{ handle_err, log_err },
        tui::{ App, TuiPrefs }
};
use super::page::PageType;

// TODO: Better error handling, remove unwrap() everywhere.

// Initialize TUI.
pub fn init_tui<'a>(nc: Arc<Mutex<&'a mut Nc>>, tui_prefs_des: &TuiPrefsDes) -> Result<App<'a>> {
    debug!("Initializing TUI.");
    let mut app = App::new(nc.clone(),
        handle_err!(TuiPrefs::gen_tui_prefs(tui_prefs_des),
            "Failed to generate TUI prefs."
        )?
    )?;

    // DEV
    app.add_page(PageType::SubredditListing)?;

    handle_err!(app.render(), "Unable to render app")?;
    Ok(app)
}
