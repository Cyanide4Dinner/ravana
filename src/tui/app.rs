use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    Nc,
    NcPlane,
    NcPlaneOptions
};
use log::{ error, info };
use std::sync::{ Arc, Mutex };

use crate::tui::TuiPrefs;
use super::subreddit_listing_page::SubListPage;
use super::{ page::{ Page, PageType },
                subreddit_listing_page::{ SubListPostData },
                util::new_child_plane,
                TuiWidget };

pub struct App<'a> {
        nc: Arc<Mutex<&'a mut Nc>>,
        plane: &'a mut NcPlane,
        tui_prefs: TuiPrefs,
        pub pages: Vec<Box<dyn Page + 'a>>
}

impl<'a> App<'a> {
    pub fn new<'b>(nc: Arc<Mutex<&'b mut Nc>>, tui_prefs: TuiPrefs) -> Result<App<'b>> {
        let mut nc_lock = nc.lock().unwrap();
        let stdplane = unsafe { nc_lock.stdplane() }; 
        let (dim_x, dim_y) = nc_lock.term_dim_yx();
        drop(nc_lock);

        Ok(
            App {
                nc: nc,
                plane: new_child_plane!(stdplane, 0, 0, dim_x, dim_y),
                tui_prefs: tui_prefs,
                pages: Vec::new()
            }
        )
    }

    pub fn add_page(&mut self, page_type: PageType) -> Result<()> {
        match page_type {
            PageType::SubredditListing => {
                let mut sub_list_page = SubListPage::new(&self.tui_prefs,
                                                             new_child_plane!(
                                                                self.plane,
                                                                0,
                                                                0,
                                                                self.plane.dim_x(),
                                                                self.plane.dim_y()
                                                                )
                                                            )?;
                sub_list_page.add_post(&self.tui_prefs, SubListPostData {
                    heading: "hadfafda",
                    content: "fahfaljdf",
                    upvotes: 78,
                    username: "afhaldjf"
                })?;
                self.pages.push(sub_list_page);
            },
            PageType::Post => {  }
        }
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        for page in self.pages.iter_mut() {
            page.draw()?;
        }

        if let Ok(mut nc_lock) = self.nc.lock() {
            nc_lock.render().context("Nc render failed.")?;
            info!("Rendered app.");
            return Ok(())
        }
        error!("Failed to render app: unable to lock Nc.");
        Err(anyhow!("Failed to render app: unable to lock Nc."))
    }
}

impl TuiWidget for App<'_> {  }
