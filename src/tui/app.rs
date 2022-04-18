use anyhow::{ anyhow, bail, Context, Result };
use libnotcurses_sys::{
    c_api::ncreader_offer_input,
    Nc,
    NcInput,
    NcMiceEvents,
    NcPlane,
    NcPlaneOptions
};
use log::{ debug, error, info };
use std::sync::{ Arc, Mutex };
use tokio::sync::oneshot;
use tokio::sync::mpsc::Sender;

use crate::{ 
        input::{
            input_message::InputMessage,
            command_to_event
        },
        state::Message,
        tools::{ handle_err, log_err }, tui::TuiPrefs 
};
use super::subreddit_listing_page::SubListPage;
use super::{CmdPalette,
                page::{ Page, PageType },
                subreddit_listing_page::SubListPostData,
                util::new_child_plane,
                Widget };

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

        // For sending messages to trigger events.
        mpsc_send: Sender<Message>,

        // Pages currently in the application.
        pub pages: Vec<Box<dyn Page + 'a>>,

        // Command palette widget.
        cmd_plt: CmdPalette<'a>
}

impl<'a> App<'a> {
    pub fn new<'b>(nc: Arc<Mutex<&'b mut Nc>>, tui_prefs: TuiPrefs, mpsc_send: Sender<Message>) 
            -> Result<App<'b>> {
        let mut nc_lock = nc.lock().unwrap();
        let stdplane = unsafe { nc_lock.stdplane() }; 
        let (dim_y, dim_x) = nc_lock.term_dim_yx();

        if tui_prefs.interface.mouse_events_enable { 
            info!("Enabling mice events.");
            handle_err!(nc_lock.mice_enable(NcMiceEvents::All), "Failed to enable mice events")?;
        }

        drop(nc_lock);

        let app_plane = new_child_plane!(stdplane, 0, 0, dim_x, dim_y);

        debug!("Creating command palette.");
        let cmd_plt = log_err!(
            CmdPalette::new(&tui_prefs,
                              app_plane,
                              0,
                              (stdplane.dim_y() - 1) as i32,
                              stdplane.dim_x(),
                              1,
                              mpsc_send.clone()
                              )
        )?;

        Ok(
            App {
                nc,
                plane: app_plane,
                tui_prefs,

                mpsc_send,

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
                let mut sub_list_page = log_err!(SubListPage::new(&self.tui_prefs,
                                                            self.plane,
                                                            0,
                                                            0,
                                                            self.plane.dim_x(),
                                                            self.plane.dim_y(),
                                                            self.mpsc_send.clone()
                                                            ))?;

                // DEV
                sub_list_page.add_post(&self.tui_prefs, SubListPostData {
                    heading: "hadfafda",
                    content: "fahfaljdf",
                    upvotes: 78,
                    username: "afhaldjf",
                    subreddit_name: "rust",
                    comments: 78
                }).context("Failed to create new page of type SubredditListing.")?;
                self.pages.push(Box::new(sub_list_page));

                // DEV
                // TODO: Find ways to move cmd_plt to top automatically.
                self.cmd_plt.plane.move_top();
            }
        }
        Ok(())
    }

    // TODO: Find better ways of ordering planes as layers in App.
    pub async fn input_cmd_plt(&mut self, ncin: NcInput, oneshot_tx: oneshot::Sender<InputMessage>) -> Result<()> {
        log_err!(self.cmd_plt.input(ncin, oneshot_tx).await)?;
        // command_to_event::exec_cmd(None, cmd).await;
        self.render()
    }

    pub fn enter_cmd(&mut self) -> Result<()> {
        debug!("Entering CmdMode.");
        // Put : in CmdPalette
        unsafe { ncreader_offer_input(self.cmd_plt.reader, &NcInput::new(':')) };
        self.render()
    }

    pub fn exit_cmd(&mut self) -> Result<()> {
        debug!("Exiting CmdMode.");
        self.cmd_plt.clear_contents();
        self.render()
    }

    pub async fn exec_cmd(&mut self) -> Result<()> {
        debug!("Executing command: {:?}", self.cmd_plt.contents()?);
        let mut cmd = self.cmd_plt.contents()?;
        command_to_event::exec_cmd(self.mpsc_send.clone(), None, &cmd.split_off(1)).await;
        Ok(())
    }

    // Render TUI.
    pub fn render(&mut self) -> Result<()> {
        for page in self.pages.iter_mut() {
            handle_err!(page.draw(&self.tui_prefs), "Failed to render page")?;
        }

        if let Ok(mut nc_lock) = self.nc.lock() {
            handle_err!(nc_lock.render(), "Failed to render app")?;
            return Ok(())
        }
        error!("Failed to render app: unable to lock Nc.");
        bail!("Failed to render app: unable to lock Nc.")
    }
}

// -----------------------------------------------------------------------------------------------------------
// * Stop Nc.
// * Destroy App plane, which should destroy planes of all children widgets, since all children
//   planes form a tree.
// -----------------------------------------------------------------------------------------------------------
impl<'a> Drop for App<'a> {
    fn drop(&mut self) {
        debug!("Dropping App.");

        // Destroy ncreader before destroying base plane or Nc instance.
        self.cmd_plt.destroy_reader();

        handle_err!(self.plane.destroy(), "Failed to destroy app plane").unwrap();

        if let Ok(mut nc_lock) = self.nc.lock() {
            unsafe { 
                handle_err!(nc_lock.stop(), "Failed to destroy Nc instance").unwrap()
            }
        } else { error!("Error locking Nc instance while dropping App."); }
    }
}
