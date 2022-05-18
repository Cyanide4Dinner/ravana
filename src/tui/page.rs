use anyhow::Result;
use libnotcurses_sys::{ 
    NcChannels,
    NcPlane,
    NcPlaneOptions
};

use super::{ TuiPrefs, util::{ new_child_plane, PostData, Widget} };

#[derive(Debug)]
pub enum PageType {
    SubredditListing
}

// -----------------------------------------------------------------------------------------------------------
// Page encapsulate functionality required to function as a "page".
// -----------------------------------------------------------------------------------------------------------
pub trait Page: Send {

    // -------------------------------------------------------------------------------------------------------
    // * Set visibility of page.
    // * If not visible, the page is shifted to right by the length of width, making it invisible
    //   on render. 
    // * For making it visible again, the shift is undoed.
    // -------------------------------------------------------------------------------------------------------
    fn set_visibility(&mut self, visible: bool) -> Result<()>;

    // Scroll up / down.
    fn scroll_up(&mut self) -> Result<()>;
    fn scroll_down(&mut self) -> Result<()>;

    // Listing functions.
    fn add_post(&mut self, tui_prefs: &TuiPrefs, data: PostData) -> Result<()>;

    // Draw widgets onto plane.
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;

    // Fetch data.
    fn fetch(&mut self) -> Result<()>;
}

// -----------------------------------------------------------------------------------------------------------
// PageBar widget
// * Shows the currnet page and list of pages on top.
// -----------------------------------------------------------------------------------------------------------
pub struct PageBar<'a> {
    pub plane: &'a mut NcPlane,
    pub foc_page: u32,
    page_names: Vec<&'a str>
}

impl<'a> Widget for PageBar<'a> {
    fn new(tui_prefs: &TuiPrefs,
                    parent_plane: &mut NcPlane,
                    x: i32,
                    y: i32,
                    dim_x: u32,
                    dim_y: u32
                   ) -> Result<Self> {
        let plane = new_child_plane!(parent_plane, x, y, dim_x, dim_y);
        plane.set_base(
            " ",
            0,
            NcChannels::from_rgb(
                tui_prefs.theme.page_bar_fg.to_nc_rgb(),
                tui_prefs.theme.page_bar_bg.to_nc_rgb(),
            ))?;

        Ok(PageBar {
            plane,
            foc_page: 0,
            page_names: vec!("hello", "yoyou")
        })
    }

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        const PAGE_NAME_WIDTH: u32 = 7;
        for (pos, e) in self.page_names.iter().enumerate() {
            if ((pos as u32) + 1) * PAGE_NAME_WIDTH < self.plane.dim_x() {
                self.plane.putstr_yx(Some(0), Some((pos as u32) * PAGE_NAME_WIDTH),
                    &format!("{}:{}", pos, &e[0..4])
                )?;
            }
        }
        let current_page_chnls = NcChannels::from_rgb(
            tui_prefs.theme.page_bar_fg.to_nc_rgb(),
            tui_prefs.theme.page_bar_current_bg.to_nc_rgb()
        );
        self.plane.stain(
            Some(0),
            Some(self.foc_page * PAGE_NAME_WIDTH),
            Some(1),
            Some(PAGE_NAME_WIDTH),
            current_page_chnls,
            current_page_chnls,
            current_page_chnls,
            current_page_chnls,
        )?;
        Ok(())
    }
}
