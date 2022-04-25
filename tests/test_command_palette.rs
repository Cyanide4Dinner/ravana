use anyhow::Result;
use libnotcurses_sys::Nc;
use std::sync::{ Mutex, Arc };

use ravana::{
    jobs::config::load_config,
    tui::{ App, PageType, TuiPrefs }
};

// -----------------------------------------------------------------------------------------------------------
// - Command palette should be at the top of App in CmdMode.
// -----------------------------------------------------------------------------------------------------------
#[test]
fn test_cmd_plt_pos() -> Result<()> {
    let config = load_config();
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let mut app = App::new(nc.clone(),
        TuiPrefs::gen_tui_prefs(&config.tui)?
    )?;
    app.add_page(PageType::SubredditListing).unwrap();
    app.render().unwrap();


    Ok(())
}
