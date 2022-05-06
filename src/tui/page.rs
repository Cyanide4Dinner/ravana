use anyhow::Result;

use super::TuiPrefs;

#[derive(Debug)]
pub enum PageType {
    SubredditListing
}

// -----------------------------------------------------------------------------------------------------------
// Page encapsulate functionality required to function as a "page".
// -----------------------------------------------------------------------------------------------------------
pub trait Page: Send {
    // Scroll up / down.
    fn scroll_up(&mut self) -> Result<()>;
    fn scroll_down(&mut self) -> Result<()>;

    // Draw widgets onto plane.
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;

    // Fetch data.
    fn fetch(&mut self) -> Result<()>;
}
