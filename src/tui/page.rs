use anyhow::Result;

use super::TuiPrefs;

pub enum PageType {
    Post,
    SubredditListing
}

pub trait Page: Send {
    // Draw widgets onto plane.
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;

    // Fetch data.
    fn fetch(&mut self) -> Result<()>;
}
