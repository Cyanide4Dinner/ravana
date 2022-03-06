use anyhow::Result;
use libnotcurses_sys::Nc;
use std::sync::{ Arc, Mutex };

use crate::{ jobs::TuiPrefsDes, 
        tui::{ App, TuiPrefs } 
};
use super::page::PageType;

// TODO: Better error handling, remove unwrap() everywhere.
pub fn init_tui<'a>(nc: Arc<Mutex<&'a mut Nc>>, tui_prefs_des: &TuiPrefsDes) -> Result<App<'a>> {
    let mut app = App::new(nc.clone(), TuiPrefs::gen_tui_prefs(tui_prefs_des)?)?;
    app.add_page(PageType::SubredditListing)?;
    app.render()?;
    Ok(app)
}
