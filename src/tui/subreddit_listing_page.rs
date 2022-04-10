use anyhow::{ Context, Result };
use libnotcurses_sys::{
    NcChannel,
    NcChannels,
    NcPlane,
    NcPlaneOptions,
    c_api
};
use log::error;

use super::{  page::Page, TuiPrefs, Widget, util::new_child_plane };

// Data to display in a post item of subreddit listing.
pub struct SubListPostData<'a> {
    pub upvotes: u32,
    pub heading: &'a str,
    pub content: &'a str,
    pub username: &'a str,
    pub subreddit_name: &'a str,
    pub comments: u32
}

// Subreddit lisitng post item widget.
pub struct SubListPost<'a> {
    plane: &'a mut NcPlane,
    data: SubListPostData<'a>
}

impl<'a> SubListPost<'a> {
    fn draw_header(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        let header_bg_channel = NcChannel::from_rgb(tui_prefs.theme.post_header_bg.to_nc_rgb());
        let header_fg_channel = NcChannel::from_rgb(tui_prefs.theme.post_header_fg.to_nc_rgb());

        let upvoted_channel = NcChannels::from_rgb(
            tui_prefs.theme.post_upvoted_fg.to_nc_rgb(),
            tui_prefs.theme.post_upvoted_bg.to_nc_rgb()
        );

        let header_combined_channel = NcChannels::combine(header_fg_channel, header_bg_channel);

        const UPVOTE_COUNT_DECIMAL_PRECISION: u32 = 7;
        const MAX_USERNAME_LEN: u32 = 16;
        const COMMENT_COUNT_DECIMAL_PRECISION: u32 = 8;

        // Fill space as character to get color on whole line.
        // TODO: Find efficient methods, use notcurses built in tools.
        self.plane.putstr_yx_stained(0, 0, &(0..self.plane.dim_x()).map(|_| " ").collect::<String>())?;

        let mut pos = 0;
        self.plane.putstr_yx(Some(0), Some(pos), &self.data.upvotes.to_string())?;

        pos = UPVOTE_COUNT_DECIMAL_PRECISION + 1;
        self.plane.putstr_yx_stained(0, pos, &self.data.username)?;

        pos = pos + MAX_USERNAME_LEN + 1;
        self.plane.putstr_yx_stained(0, pos, &self.data.subreddit_name)?;

        pos = self.plane.dim_x() - COMMENT_COUNT_DECIMAL_PRECISION + 1;
        self.plane.putstr_yx_stained(0, pos, &self.data.comments.to_string())?;

        self.plane.stain(
            Some(0),
            Some(0),
            Some(1),
            None,
            header_combined_channel,
            header_combined_channel,
            header_combined_channel,
            header_combined_channel,
        )?;

        // If upvoted, indicate by different color.
        let upvoted = true;
        if upvoted {
            self.plane.stain(
                Some(0),
                Some(0),
                Some(1),
                Some(UPVOTE_COUNT_DECIMAL_PRECISION - 1),
                upvoted_channel,
                upvoted_channel,
                upvoted_channel,
                upvoted_channel,
            )?;
        }
        Ok(())
    }

    fn draw_heading(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        let heading_bg_channel = NcChannel::from_rgb(tui_prefs.theme.post_heading_bg.to_nc_rgb());
        let heading_fg_channel = NcChannel::from_rgb(tui_prefs.theme.post_heading_fg.to_nc_rgb());

        let heading_combined_channel = NcChannels::combine(heading_fg_channel, heading_bg_channel);

        // Fill space as character to get color on whole line.
        // TODO: Find efficient methods, use notcurses built in tools.
        self.plane.putstr_yx_stained(1, 0, &(0..self.plane.dim_x()).map(|_| " ").collect::<String>())?;

        self.plane.putnstr_yx(Some(1), Some(0), self.plane.dim_x() as usize, self.data.heading)?;
        self.plane.stain(
            Some(1),
            Some(0),
            Some(1),
            None,
            heading_combined_channel,
            heading_combined_channel,
            heading_combined_channel,
            heading_combined_channel,
        )?;

        // Make heading bold formatted.
        self.plane.format(Some(1), Some(0), Some(1), None, c_api::NCSTYLE_BOLD)?;
        Ok(())
    }
}

impl<'a> Widget for SubListPost<'a> {
    fn new(tui_prefs: &TuiPrefs,
                    parent_plane: &mut NcPlane,
                    x: i32,
                    y: i32,
                    dim_x: u32,
                    dim_y: u32
                   ) -> Result<Self> {
        let plane = new_child_plane!(parent_plane, x, y, dim_x, dim_y);

        Ok(Self {
                plane,
                data: SubListPostData {
                    heading: "Heading 1",
                    content: "",
                    upvotes: 18901,
                    username: "AyeDeeKay",
                    subreddit_name: "Rust",
                    comments: 17
                }
        })
    }

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        self.draw_header(tui_prefs)?;
        self.draw_heading(tui_prefs)
    }
}

// -----------------------------------------------------------------------------------------------------------
// Page for displaying subreddit listing.
// -----------------------------------------------------------------------------------------------------------
pub struct SubListPage<'a> {
    plane: &'a mut NcPlane,
    posts: Vec<SubListPost<'a>>
}

impl<'a> SubListPage<'a> {
    pub fn add_post(&mut self, tui_prefs: &TuiPrefs, data: SubListPostData<'a>) -> Result<()> {
        self.posts.push(SubListPost::new(
                tui_prefs,
                self.plane,
                0,
                0,
                self.plane.dim_x(),
                self.plane.dim_y()
            )?);
        Ok(())
    }
}

impl<'a> Widget for SubListPage<'a> {
    fn new(tui_prefs: &TuiPrefs,
                   parent_plane: &mut NcPlane,
                   x: i32,
                   y: i32,
                   dim_x: u32,
                   dim_y: u32
                   ) -> Result<Self> {
        let plane = new_child_plane!(parent_plane, x, y, dim_x, dim_y);

        plane.set_fchannel(NcChannel::from_rgb(tui_prefs.theme.highlight_fg.to_nc_rgb()));
        plane.set_bchannel(NcChannel::from_rgb(tui_prefs.theme.highlight_bg.to_nc_rgb()));
        
        Ok(Self { 
            plane,
            posts: vec![]
        })
    }

    fn draw(&mut self, _tui_prefs: &TuiPrefs) -> Result<()> {
        Ok(())
    }
}

impl Page for SubListPage<'_> {
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        for post in self.posts.iter_mut() {
            post.draw(tui_prefs).context("Failed to render post.")?;
        }
        Ok(())
    }

    fn fetch(&mut self) -> Result<()> {
        Ok(())
    }
}
