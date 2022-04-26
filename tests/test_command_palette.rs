use anyhow::Result;
use env_logger;
use libnotcurses_sys::{ Nc, NcChannels, NcStyle };
use log::{ debug, info, error };
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
    env_logger::init();
    let mut config = load_config();
    config.tui.interface.mouse_events_enable = false;
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let mut app = App::new(nc.clone(),
        TuiPrefs::gen_tui_prefs(&config.tui)?
    )?;
    app.dummy_render()?;
    app.render().unwrap();
    
    let mut stylemask = NcStyle(0);
    let mut channels = NcChannels::new();

    let mut nc_lock = nc.lock().unwrap();
    let s = nc_lock.at_yx(1, 0, &mut stylemask, &mut channels);
    drop(nc_lock);

    assert_eq!(s, Some("H".to_string()));

    Ok(())
}
