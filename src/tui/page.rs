use anyhow::Result;
use libnotcurses_sys::{ 
    NcCell,
    NcChannel,
    NcChannels,
    NcPlane,
    NcPlaneOptions
};

use super::{ TuiPrefs, util::{ new_child_plane, Widget} };

#[derive(Debug)]
pub enum PageType {
    SubredditListing
}

// -----------------------------------------------------------------------------------------------------------
// Page encapsulate functionality required to function as a "page".
// -----------------------------------------------------------------------------------------------------------
pub trait Page: Send {
    // Scroll up / down.
    fn scroll_up(&mut self) -> Result<()>;
    fn scroll_down(&mut self) -> Result<()>;

    // Draw widgets onto plane.
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;

    // Fetch data.
    fn fetch(&mut self) -> Result<()>;
}

pub struct PageBar<'a> {
    pub plane: &'a mut NcPlane,
    foc_page: u32,
    page_names: Vec<&'a str>
}

impl<'a> Widget for PageBar<'a> {
    fn new(_tui_prefs: &TuiPrefs,
                    parent_plane: &mut NcPlane,
                    x: i32,
                    y: i32,
                    dim_x: u32,
                    dim_y: u32
                   ) -> Result<Self> {
        let plane = new_child_plane!(parent_plane, x, y, dim_x, dim_y);
        plane.set_base_cell(&NcCell::from_char7b(' ')?)?;

        Ok(PageBar {
            plane,
            foc_page: 0,
            page_names: vec!("hello", "yoyou")
        })
    }

    fn draw(&mut self, _tui_prefs: &TuiPrefs) -> Result<()> {
        const PAGE_NAME_WIDTH: u32 = 7;
        for (pos, e) in self.page_names.iter().enumerate() {
            if ((pos as u32) + 1) * PAGE_NAME_WIDTH < self.plane.dim_x() {
                self.plane.putstr_yx(Some(0), Some((pos as u32) * PAGE_NAME_WIDTH),
                    &format!("{}:{}", pos, &e[0..4])
                )?;
            }
        }
        let chnls = NcChannels::from_rgb(
            _tui_prefs.theme.cmd_plt_fg.to_nc_rgb(),
            _tui_prefs.theme.cmd_plt_bg.to_nc_rgb()
        );
        self.plane.stain(
            Some(0),
            Some(self.foc_page * PAGE_NAME_WIDTH),
            Some(1),
            Some(PAGE_NAME_WIDTH),
            chnls,
            chnls,
            chnls,
            chnls
        )?;
        Ok(())
    }
}
