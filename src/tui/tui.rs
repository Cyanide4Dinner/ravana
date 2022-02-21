use anyhow::Result;
use libnotcurses_sys::Nc;
use std::sync::{ Arc, Mutex };

use crate::tui::{ App, PageType };

// TODO: Better error handling, remove unwrap() everywhere.
pub fn init_tui(nc: Arc<Mutex<&mut Nc>>) -> Result<App> {
    let mut app = App::new(nc.clone());
    app.add_page(PageType::SubredditListing);
    app.render()?;
    Ok(app)
}
