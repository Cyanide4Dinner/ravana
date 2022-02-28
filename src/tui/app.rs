use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    Nc,
    NcAlign,
    NcPlane,
    NcPlaneOptions
};
use log::{ error, info };
use std::sync::{ Arc, Mutex };

use crate::tui::TuiPrefs;
use super::subreddit_listing_page::SubListPage;
use super::page::{ Page, PageProps, PageType };

pub struct App<'a> {
   nc: Arc<Mutex<&'a mut Nc>>,
   tui_prefs: TuiPrefs,
   app_plane: &'a mut NcPlane,
   pub dim_x: u32, 
   pub dim_y: u32,
   pub pages: Vec<Box<dyn Page>>
}

impl<'a> App<'a> {
    pub fn new<'b>(nc: Arc<Mutex<&'b mut Nc>>, tui_prefs: TuiPrefs) -> App<'b> {
        let mut nc_lock = nc.lock().unwrap();
        let stdplane = unsafe { nc_lock.stdplane() }; 
        let (dim_x, dim_y) = nc_lock.term_dim_yx();
        drop(nc_lock);

        App {
            nc: nc,
            tui_prefs: tui_prefs,
            app_plane: stdplane,
            dim_x: dim_x,
            dim_y: dim_y,
            pages: Vec::new()
        }
    }

    pub fn add_page(&mut self, page_type: PageType) -> Result<()> {
        self.pages.push(SubListPage::new(self.app_plane, PageProps { dim_x: self.dim_x, dim_y: self.dim_y })?);
        Ok(())
    }

    pub fn render(&mut self) -> Result<()> {
        if let Ok(mut nc_lock) = self.nc.lock() {
            nc_lock.render().context("Nc render failed.")?;
            info!("Rendered app.");
            return Ok(())
        }
        error!("Failed to render app: unable to lock Nc.");
        Err(anyhow!("Failed to render app: unable to lock Nc."))
    }
}
