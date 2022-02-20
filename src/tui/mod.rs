mod tui;
mod app;
mod page;

pub use page::{ Page, PageType };
pub use tui::init_tui;
pub use app::App;

pub mod subreddit_listing_page;
