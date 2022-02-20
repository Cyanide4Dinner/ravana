use anyhow::{ anyhow, Context, Result }; 
use libnotcurses_sys::{ 
    NcPlane, 
    NcPlaneOptions 
};

use super::subreddit_listing_page::{ SubListPage, SubListPageProps };
use super::App;

pub enum PageType {
    Post,
    SubredditListing
}

// Page properties.
pub struct PageProps {
    pub dim_x: u32,
    pub dim_y: u32
}

// Page has pointers to structures corresponding to various types of pages.
// They will each be passed the Nc pointer.
// Then each page structure can produce TUI and associated functionality accordingly.
pub struct Page<'a> {
    // nc: Arc<Mutex<&'a mut Nc>>,
    pub page_type: PageType,
    post_page: Option<Box<PostPage<'a>>>,
    listing_page: Option<Box<SubListPage<'a>>>
}

impl<'a> Page<'a> {
    pub fn new<'b>(app_page: &'b mut NcPlane, page_type: PageType, page_props: PageProps) -> Result<Page<'a>> {
        let page: Page;
        match page_type {
            PageType::Post => { 
                Ok(Page {
                    // nc: nc.clone(),
                    page_type : PageType::Post,
                    post_page : None,
                    listing_page : None
                })
            },
            PageType::SubredditListing => { 
                Ok(Page {
                    page_type : PageType::SubredditListing,
                    post_page : None,
                    listing_page : Some(SubListPage::new(app_page, SubListPageProps { dim_x: page_props.dim_x, dim_y: page_props.dim_y })?)
                })
            }
            _ => { Err(anyhow!("Page type not found")) }
        }
    }
}

pub struct PostPage<'a> {
    base_plane: &'a mut NcPlane
}
impl<'a> PostPage<'a> {
    pub fn new<'b>(app_page: &'b mut NcPlane) -> Result<Box<PostPage<'a>>> {
        let base_plane = NcPlane::new_child(app_page, &NcPlaneOptions::new(0, 0, 20u32, 20u32))?;
        // base_plane.putstr("Yo Yo fkadjfla djfkladf ")?;
        Ok(Box::new(PostPage { base_plane: base_plane }))
    }

}
