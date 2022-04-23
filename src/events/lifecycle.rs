use anyhow::{ anyhow, Result };
use libnotcurses_sys::Nc;
use log::{ error };
use std::sync::{ Mutex, Arc };

use crate::{
    input::{ listen, create_key_bindings_trie },
    jobs::config::load_config,
    tools::{ log_err_desc_ret, log_err_ret },
    tui::{ App, page::PageType, TuiPrefs, val_tui_prefs_des },
};

// -----------------------------------------------------------------------------------------------------------
// * Main loop.
// * Input --> Process --> State --> TUI --> Input.
// -----------------------------------------------------------------------------------------------------------
pub fn ravana() -> Result<()> {
    let config = load_config();

    // Instantiating Nc instance.
    let nc = Arc::new(Mutex::new(unsafe { log_err_desc_ret!(Nc::new(), "Failed to instantiate Nc.")? }));

    // Validate config
    // TODO: Validate key-bindings
    {
        if !val_tui_prefs_des(&config.tui) { 
            return log_err_ret!(Err(anyhow!("Invalid TUI format in config.")));
        } 
    }

    let mut app = App::new(nc.clone(),
        log_err_desc_ret!(TuiPrefs::gen_tui_prefs(&config.tui),
            "Failed to generate TUI prefs"
        )?
    )?;
    app.add_page(PageType::SubredditListing).unwrap();
    app.render().unwrap();

    let kbt = log_err_desc_ret!(create_key_bindings_trie(&config.key_bindings), "Failed to create KB trie")?;

    listen(nc, kbt, &mut app).unwrap();

    Ok(())
}
