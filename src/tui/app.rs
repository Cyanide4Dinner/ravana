use anyhow::{ anyhow,  Context, Result };
use libnotcurses_sys::{
    c_api::ncreader_offer_input,
    Nc,
    NcInput,
    NcMiceEvents,
    NcPlane,
    NcPlaneOptions
};
use log::{ error, info };
use ravana_reddit_api::api::RedditClient;
use std::sync::{ Arc, Mutex };
use tokio::runtime;

use crate::{ 
        input::command_to_event,
        tools::{ log_err_desc_ret, log_err_ret }, 
        tui::{ AppRes, TuiPrefs }
};
use super::subreddit_listing_page::SubListPage;
use super::{ 
        command_palette::CmdPalette,
        page::{ Page, PageBar, PageType },
        util::new_child_plane,
        util::Widget,
        util::PostData
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
        // Runtime object for executing async.
        rt: runtime::Runtime,
        reddit_client: RedditClient,

        plane: &'a mut NcPlane,
        tui_prefs: TuiPrefs,

        // Pages currently in the application.
        foc_page: usize,
        pub pages: Vec<Box<dyn Page + 'a>>,
        pub page_bar: PageBar<'a>,

        // Command palette widget.
        pub cmd_plt: CmdPalette<'a>
}

impl<'a> App<'a> {
    pub fn new<'b>(nc: Arc<Mutex<&'b mut Nc>>, tui_prefs: TuiPrefs, reddit_client: RedditClient) 
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
                              (app_plane.dim_y() - 1) as i32,
                              app_plane.dim_x(),
                              1
                              )
        )?;

        let page_bar = PageBar::new(&tui_prefs,
                                       app_plane,
                                       0,
                                       0,
                                       app_plane.dim_x(),
                                       1)?;

        Ok(
            App {
                nc,
                rt: runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()?,
                reddit_client,

                plane: app_plane,
                tui_prefs,

                foc_page: 0,
                pages: Vec::new(),
                page_bar,

                cmd_plt
            }
        )
    }

    // Add a new page.
    pub fn add_page(&mut self, page_type: PageType) -> Result<()> {
        info!("Adding new page of type {:?}.", page_type);
        match page_type {
            PageType::SubredditListing => {
                let sub_list_page = log_err_ret!(SubListPage::new(&self.tui_prefs,
                                                            self.plane,
                                                            0,
                                                            1,
                                                            self.plane.dim_x(),
                                                            self.plane.dim_y() - 1,
                                                            ))?;
                self.pages.push(Box::new(sub_list_page));
                self.foc_page = self.pages.len() - 1;
            }
        }
        Ok(())
    }

    pub fn dummy_render(&mut self) -> Result<()> {
        self.add_page(PageType::SubredditListing)?;
        let sub_list_page = &mut self.pages[0];       
        for x in 0..13 {
            (*sub_list_page).add_post(&self.tui_prefs, PostData {
                heading: "hadfafda",
                content: "fahfaljdf",
                upvotes: x,
                username: "afhaldjf",
                subreddit_name: "rust",
                comments: 78,
                body: "jfkladjfl ajdfla jdflkj"
            }).context("Failed to create new page of type SubredditListing.")?;
        }

        self.add_page(PageType::SubredditListing)?;
        let sub_list_page2 = &mut self.pages[1]; 
        
        for x in 0..13 {
            (*sub_list_page2).add_post(&self.tui_prefs, PostData {
                heading: "ffffff",
                content: "hhhhhhhh",
                upvotes: x,
                username: "bbbbbbbbb",
                subreddit_name: "hhhhhhhh",
                comments: 78,
                body: "ooooooooooooooooooo"
            }).context("Failed to create new page of type SubredditListing.")?;
        }

        self.foc_page = 1;

        self.cmd_plt.plane.move_top();
        self.page_bar.plane.move_top();

        Ok(())
    }

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

    pub fn switch_next_page(&mut self) {
        self.set_foc_page((self.foc_page + 1) % self.pages.len());
    }

    pub fn switch_prev_page(&mut self) {
        if self.foc_page == 0 {
            self.set_foc_page(self.pages.len() - 1);
        } else {
            self.set_foc_page(self.foc_page - 1);
        }
    }

    pub fn set_foc_page(&mut self, new_foc_page: usize) {
        self.foc_page = new_foc_page;
        self.page_bar.foc_page = new_foc_page as u32;
    }

    pub fn scroll_up(&mut self) { 
        if let Err(e) = (*self.pages[self.foc_page]).scroll_up() {
            error!("{}", e);
        }
    }

    pub fn scroll_down(&mut self) { 
        if let Err(e) = (*self.pages[self.foc_page]).scroll_down() {
            error!("{}", e)
        }
    }

    // Render TUI.
    pub fn render(&mut self) -> Result<()> {
        self.page_bar.draw(&self.tui_prefs)?;

        for page in self.pages.iter_mut() {
            log_err_desc_ret!(page.draw(&self.tui_prefs), "Failed to render page")?;
            page.set_visibility(false)?;
        }

        self.pages[self.foc_page].set_visibility(true)?;

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
