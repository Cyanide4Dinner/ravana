use anyhow::Result;
use libnotcurses_sys::{
    NcAlign,
    NcChannel,
    NcChannels,
    NcPlane,
    NcPlaneOptions,
    NcStyle
};
use log::error;

use crate::tools::log_err_desc_ret;
use super::{ page::Page, TuiPrefs, util::{ Color, new_child_plane, Widget } };

// Data to display in a post item of subreddit listing.
pub struct SubListPostData<'a> {
    pub upvotes: u32,
    pub heading: &'a str,
    pub content: &'a str,
    pub username: &'a str,
    pub subreddit_name: &'a str,
    pub comments: u32,
    pub body: &'a str
}

// Subreddit lisitng post item widget.
pub struct SubListPost<'a> {
    plane: &'a mut NcPlane,

    data: SubListPostData<'a>,

    hdr_plane:  &'a mut NcPlane,
    hdg_plane:  &'a mut NcPlane,
    body_plane: &'a mut NcPlane
}

impl<'a> SubListPost<'a> {
    fn draw_header(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        let upvoted_channel = NcChannels::from_rgb(
            tui_prefs.theme.post_upvoted_fg.to_nc_rgb(),
            tui_prefs.theme.post_upvoted_bg.to_nc_rgb()
        );

        const UPVOTE_COUNT_DECIMAL_PRECISION: u32 = 7;
        const MAX_USERNAME_LEN: u32 = 16;
        const COMMENT_COUNT_DECIMAL_PRECISION: u32 = 8;

        let mut pos = 0;
        self.hdr_plane.putstr_yx_stained(0, pos, &self.data.upvotes.to_string())?;

        pos = UPVOTE_COUNT_DECIMAL_PRECISION + 1;
        self.hdr_plane.putstr_yx(Some(0), Some(pos), &self.data.username)?;

        pos = pos + MAX_USERNAME_LEN + 1;
        self.hdr_plane.putstr_yx(Some(0), Some(pos), &self.data.subreddit_name)?;

        pos = self.plane.dim_x() - COMMENT_COUNT_DECIMAL_PRECISION + 1;
        self.hdr_plane.putstr_yx(Some(0), Some(pos), &self.data.comments.to_string())?;

        let upvoted = true;
        if upvoted {
            self.hdr_plane.stain(
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

    // TODO: Safeguard against text overflow since the App crashes.
    fn draw_heading(&mut self) -> Result<()> {
        self.hdg_plane.puttext(0, NcAlign::Left, self.data.heading)?;
        Ok(())
    }

    // TODO: Safeguard against text overflow since the App crashes.
    fn draw_body(&mut self) -> Result<()> {
        self.body_plane.puttext(0, NcAlign::Left, self.data.body)?;
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

        let body_plane = new_child_plane!(parent_plane, 0, 2, dim_x, 3);
        body_plane.set_base(
            " ",
            0,
            NcChannels::from_rgb(
                tui_prefs.theme.post_body_fg.to_nc_rgb(),
                tui_prefs.theme.post_body_bg.to_nc_rgb()
            ))?;
        body_plane.set_fg_rgb(tui_prefs.theme.post_body_fg.to_nc_rgb());
        body_plane.set_bg_rgb(tui_prefs.theme.post_body_bg.to_nc_rgb());


        let hdr_plane = new_child_plane!(parent_plane, 0, 0, dim_x, 1);
        hdr_plane.set_base(
            " ",
            0,
            NcChannels::from_rgb(
                tui_prefs.theme.post_header_fg.to_nc_rgb(),
                tui_prefs.theme.post_header_bg.to_nc_rgb()
            ))?;
        hdr_plane.set_fg_rgb(tui_prefs.theme.post_header_fg.to_nc_rgb());
        hdr_plane.set_bg_rgb(tui_prefs.theme.post_header_bg.to_nc_rgb());

        let hdg_plane = new_child_plane!(parent_plane, 0, 1, dim_x, 1);
        hdg_plane.set_base(
            " ",
            0,
            NcChannels::from_rgb(
                tui_prefs.theme.post_heading_fg.to_nc_rgb(),
                tui_prefs.theme.post_heading_bg.to_nc_rgb()
            )
        )?;
        hdg_plane.set_styles(NcStyle::Bold);
        hdg_plane.set_fg_rgb(tui_prefs.theme.post_heading_fg.to_nc_rgb());
        hdg_plane.set_bg_rgb(tui_prefs.theme.post_heading_bg.to_nc_rgb());

        Ok(Self {
                plane,

                data: SubListPostData {
                    heading: "Heading",
                    content: "",
                    upvotes: 18901,
                    username: "AyeDeeKay",
                    subreddit_name: "Rust",
                    comments: 17,
                    body: "akjfldajf lajdfl jadlf jald fjla jdfla jjadlf jald fjla jdfla jfdjadlf jald fjla jdfla jfdjadlf jald fjla jdfla jfdjadlf jald fjla jdfla jfdjadlf jald fjla jdfla jfdjadlf jald fjla jdfla jfdjadlf jald fjla jdfla jfdfd"
                },

                hdr_plane,
                hdg_plane,
                body_plane

        })
    }

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        log_err_desc_ret!(self.draw_header(tui_prefs), "Failed to draw header.")?;
        log_err_desc_ret!(self.draw_heading(), "Failed to draw heading.")?;
        log_err_desc_ret!(self.draw_body(), "Failed to draw body.")
    }
}

// -----------------------------------------------------------------------------------------------------------
// Page for displaying subreddit listing.
// -----------------------------------------------------------------------------------------------------------
pub struct SubListPage<'a> {
    plane: &'a mut NcPlane,
    posts: Vec<SubListPost<'a>>,
}

impl<'a> SubListPage<'a> {
    pub fn add_post(&mut self, tui_prefs: &TuiPrefs, _data: SubListPostData<'a>) -> Result<()> {
        self.posts.push(SubListPost::new(
                tui_prefs,
                self.plane,
                0,
                0,
                self.plane.dim_x(),
                self.plane.dim_y(),
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
                   dim_y: u32,
                   ) -> Result<Self> {
        let plane = new_child_plane!(parent_plane, x, y, dim_x, dim_y);

        plane.set_fchannel(NcChannel::from_rgb(tui_prefs.theme.highlight_fg.to_nc_rgb()));
        plane.set_bchannel(NcChannel::from_rgb(tui_prefs.theme.highlight_bg.to_nc_rgb()));
        
        Ok(Self { 
            plane,
            posts: vec![],
        })
    }

    fn draw(&mut self, _tui_prefs: &TuiPrefs) -> Result<()> {
        Ok(())
    }
}

impl Page for SubListPage<'_> {
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        for post in self.posts.iter_mut() {
            post.draw(tui_prefs)?;
        }
        Ok(())
    }

    fn fetch(&mut self) -> Result<()> {
        Ok(())
    }
}
