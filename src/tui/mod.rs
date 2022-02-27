mod tui;
mod app;
mod page;
mod util;

pub use page::{ Page, PageType };
pub use tui::init_tui;
pub use app::App;

pub mod subreddit_listing_page;

pub use util::{ TuiPrefs, val_tui_prefs_des };
