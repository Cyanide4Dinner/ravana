use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    Nc,
    NcInput,
    NcMiceEvents,
    NcPlane,
    NcPlaneOptions
};
use log::{ error, info };
use std::sync::{ Arc, Mutex };

use crate::tui::TuiPrefs;
use super::subreddit_listing_page::SubListPage;
use super::{CmdPalette,
                page::{ Page, PageType },
                subreddit_listing_page::SubListPostData,
                util::new_child_plane,
                Widget };

pub struct App<'a> {
        nc: Arc<Mutex<&'a mut Nc>>,
        plane: &'a mut NcPlane,
        tui_prefs: TuiPrefs,
        pub pages: Vec<Box<dyn Page + 'a>>,

        cmd_plt: CmdPalette<'a>
}

impl<'a> App<'a> {
    pub fn new<'b>(nc: Arc<Mutex<&'b mut Nc>>, tui_prefs: TuiPrefs) -> Result<App<'b>> {
        let mut nc_lock = nc.lock().unwrap();
        let stdplane = unsafe { nc_lock.stdplane() }; 
        let (dim_y, dim_x) = nc_lock.term_dim_yx();

        if tui_prefs.interface.mouse_events_enable { nc_lock.mice_enable(NcMiceEvents::All)?; }

        drop(nc_lock);

        let app_plane = new_child_plane!(stdplane, 0, 0, dim_x, dim_y);

        let cmd_plt = CmdPalette::new(&tui_prefs,
                                      app_plane,
                                      0,
                                      (stdplane.dim_y() - 1) as i32,
                                      stdplane.dim_x(),
                                      1
                                      )?;

        Ok(
            App {
                nc,
                plane: app_plane,
                tui_prefs,
                pages: Vec::new(),

                cmd_plt
            }
        )
    }

    pub fn add_page(&mut self, page_type: PageType) -> Result<()> {
        match page_type {
            PageType::SubredditListing => {
                let mut sub_list_page = SubListPage::new(&self.tui_prefs,
                                                            self.plane,
                                                            0,
                                                            0,
                                                            self.plane.dim_x(),
                                                            self.plane.dim_y()
                                                            )?;
                sub_list_page.add_post(&self.tui_prefs, SubListPostData {
                    heading: "hadfafda",
                    content: "fahfaljdf",
                    upvotes: 78,
                    username: "afhaldjf",
                    subreddit_name: "rust",
                    comments: 78
                })?;
                self.pages.push(Box::new(sub_list_page));
                self.cmd_plt.plane.move_top();
            }
        }

        Ok(())
    }

    // TODO: Find better ways of ordering planes as layers in App.
    pub fn input_cmd_plt(&mut self, ncin: NcInput) -> Result<()> {
        self.cmd_plt.input(ncin)?;
        self.render()
    }

    pub fn render(&mut self) -> Result<()> {
        for page in self.pages.iter_mut() {
            page.draw(&self.tui_prefs)?;
        }

        if let Ok(mut nc_lock) = self.nc.lock() {
            nc_lock.render().context("Nc render failed.")?;
            // info!("Rendered app.");
            return Ok(())
        }
        error!("Failed to render app: unable to lock Nc.");
        Err(anyhow!("Failed to render app: unable to lock Nc."))
    }
}

// -----------------------------------------------------------------------------------------------------------
// - Drop App -
// -----------------------------------------------------------------------------------------------------------
// - Stop Nc.
// - Destroy App plane, which should destroy planes of all children widgets, since all children
//   planes form a tree.
// -----------------------------------------------------------------------------------------------------------
impl<'a> Drop for App<'a> {
    fn drop(&mut self) {
        // for page in self.pages.iter_mut() {
        //     drop(page);
        // }
        info!("Dropping App.");

        // Destroy ncreader before destroying base plane or Nc instance.
        self.cmd_plt.destory_reader();

        if let Err(err) = self.plane.destroy() {
            error!("Error dropping App plane: {}", err);
        }
        if let Ok(mut nc_lock) = self.nc.lock() {
            unsafe { 
                if let Err(err) = nc_lock.stop() {
                    error!("Error destroying Nc instance while dropping App: {}", err);
                } 
            }
        } else { error!("Error locking Nc instance while dropping App."); }
    }
}
