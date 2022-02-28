use anyhow::Result;

pub enum PageType {
    Post,
    SubredditListing
}

// Page properties.
pub struct PageProps {
    pub dim_x: u32,
    pub dim_y: u32
}

pub trait Page: Send {
    fn draw(&self) -> Result<()>;
}
