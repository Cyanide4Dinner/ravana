use anyhow::Result;

use super::TuiPrefs;

pub enum PageType {
    Post,
    SubredditListing
}

// Drop trati needed because libnotcurses_sys doesn't call destructor methods
pub trait Page: Send + Drop {
    // Draw widgets onto plane.
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;

    // Fetch data.
    fn fetch(&mut self) -> Result<()>;
}
