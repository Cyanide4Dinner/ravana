use anyhow::Result;

pub enum PageType {
    Post,
    SubredditListing
}

pub trait Page: Send {
    // Draw widgets onto plane.
    fn draw(&mut self) -> Result<()>;

    // Fetch data.
    fn fetch(&mut self) -> Result<()>;
}
