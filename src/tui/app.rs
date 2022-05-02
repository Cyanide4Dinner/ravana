use anyhow::{ anyhow, bail, Context, Result };
use libnotcurses_sys::{
    c_api::ncreader_offer_input,
    Nc,
    NcInput,
    NcMiceEvents,
    NcPlane,
    NcPlaneOptions
};
use log::{ error, info };
use std::sync::{ Arc, Mutex };

use crate::{ 
        input::command_to_event,
        tools::{ log_err_desc_ret, log_err_ret }, 
        tui::{ AppRes, TuiPrefs }
};
use super::subreddit_listing_page::SubListPage;
use super::{ 
        command_palette::CmdPalette,
        page::{ Page, PageType },
        subreddit_listing_page::SubListPostData,
        util::new_child_plane,
        util::Widget 
};

// -----------------------------------------------------------------------------------------------------------
// * Primary base App.
// * All widgets are subordinate to this.
// * All widget planes are derived from this plane or its children.
// * It represents the current state of TUI and the application.
// -----------------------------------------------------------------------------------------------------------
pub struct App<'a> {
        // Nc instance.
        nc: Arc<Mutex<&'a mut Nc>>,
        plane: &'a mut NcPlane,
        tui_prefs: TuiPrefs,

        // Pages currently in the application.
        pub pages: Vec<Box<dyn Page + 'a>>,

        // Command palette widget.
        pub cmd_plt: CmdPalette<'a>
}

impl<'a> App<'a> {
    pub fn new<'b>(nc: Arc<Mutex<&'b mut Nc>>, tui_prefs: TuiPrefs) 
            -> Result<App<'b>> {
        let mut nc_lock = nc.lock().unwrap();
        let stdplane = unsafe { nc_lock.stdplane() }; 
        let (dim_y, dim_x) = nc_lock.term_dim_yx();

        if tui_prefs.interface.mouse_events_enable { 
            info!("Enabling mice events.");
            log_err_desc_ret!(nc_lock.mice_enable(NcMiceEvents::All), "Failed to enable mice events")?;
        }
        drop(nc_lock);

        let app_plane = new_child_plane!(stdplane, 0, 0, dim_x, dim_y);

        let cmd_plt = log_err_ret!(
            CmdPalette::new(&tui_prefs,
                              app_plane,
                              0,
                              (stdplane.dim_y() - 1) as i32,
                              stdplane.dim_x(),
                              1
                              )
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

    // Add a new page.
    pub fn add_page(&mut self, page_type: PageType) -> Result<()> {
        info!("Adding new page of type {:?}.", page_type);
        match page_type {
            PageType::SubredditListing => {
                // let mut sub_list_page = log_err_ret!(SubListPage::new(&self.tui_prefs,
                //                                             self.plane,
                //                                             0,
                //                                             0,
                //                                             self.plane.dim_x(),
                //                                             self.plane.dim_y(),
                //                                             ))?;
                //
                // // DEV
                // sub_list_page.add_post(&self.tui_prefs, SubListPostData {
                //     heading: "hadfafda",
                //     content: "fahfaljdf",
                //     upvotes: 78,
                //     username: "afhaldjf",
                //     subreddit_name: "rust",
                //     comments: 78
                // }).context("Failed to create new page of type SubredditListing.")?;
                // self.pages.push(Box::new(sub_list_page));
                //
                // // DEV
                // // TODO: Find ways to move cmd_plt to top automatically.
                // self.cmd_plt.plane.move_top();
            }
        }
        Ok(())
    }

    // TODO: Remove (simply for dev process)
    pub fn dummy_render(&mut self) -> Result<()> {
        let mut sub_list_page = log_err_ret!(SubListPage::new(&self.tui_prefs,
                                                    self.plane,
                                                    0,
                                                    0,
                                                    self.plane.dim_x(),
                                                    self.plane.dim_y() - 1
                                                    ))?;

        // DEV
        sub_list_page.add_post(&self.tui_prefs, SubListPostData {
            heading: "hadfafda",
            content: "fahfaljdf",
            upvotes: 78,
            username: "afhaldjf",
            subreddit_name: "rust",
            comments: 78,
            body: "jfkladjfl ajdfla jdflkj"
        }).context("Failed to create new page of type SubredditListing.")?;

        self.pages.push(Box::new(sub_list_page));

        // DEV
        // TODO: Find ways to move cmd_plt to top automatically.
        self.cmd_plt.plane.move_top();
        Ok(())
    }

    // TODO: Find better ways of ordering planes as layers in App.
    pub fn input_cmd_plt(&mut self, ncin: NcInput) -> Result<AppRes> {
        let res = log_err_ret!(self.cmd_plt.input(ncin))?;
        self.render()?;
        Ok(res)
    }

    pub fn enter_cmd(&mut self) -> Result<()> {
        // Put : in CmdPalette
        unsafe { ncreader_offer_input(self.cmd_plt.reader, &NcInput::new(':')) };
        self.render()
    }

    pub fn exit_cmd(&mut self) -> Result<()> {
        self.cmd_plt.clear_contents();
        self.render()
    }

    // Execute command typed in command palette.
    pub fn exec_cmd(&mut self) -> Result<Option<AppRes>> {
        let cmd = log_err_ret!(self.cmd_plt.contents())?;
        self.cmd_plt.clear_contents();
        self.render()?;
        command_to_event::exec_cmd(self, &cmd[1..cmd.len()]) // Ignore first char which is ':'
    }

    // Render TUI.
    pub fn render(&mut self) -> Result<()> {
        for page in self.pages.iter_mut() {
            log_err_desc_ret!(page.draw(&self.tui_prefs), "Failed to render page")?;
        }

        if let Ok(mut nc_lock) = self.nc.lock() {
            log_err_desc_ret!(nc_lock.render(), "Failed to render app")?;
            return Ok(())
        } else {
            return log_err_ret!(Err(anyhow!("Failed to render App: unable to lock Nc.")))
        }
    }
}

// -----------------------------------------------------------------------------------------------------------
// * Stop Nc.
// * Destroy App plane, which should destroy planes of all children widgets, since all children
//   planes form a tree.
// -----------------------------------------------------------------------------------------------------------
impl<'a> Drop for App<'a> {
    fn drop(&mut self) {

        // Destroy ncreader before destroying base plane or Nc instance.
        self.cmd_plt.destroy_reader();

        log_err_desc_ret!(self.plane.destroy(), "Failed to destroy app plane").unwrap();

        if let Ok(mut nc_lock) = self.nc.lock() {
            unsafe { 
                log_err_desc_ret!(nc_lock.stop(), "Failed to destroy Nc instance").unwrap()
            }
        } else { error!("Error locking Nc instance while dropping App."); }
    }
}
