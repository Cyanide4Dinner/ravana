use anyhow::{ bail, Result };
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
use super::{ page::Page, TuiPrefs, util::{ Group, new_child_plane, PostData, Widget } };

// Subreddit lisitng post item widget.
pub struct SubListPost<'a> {
    plane: &'a mut NcPlane,

    hdr_plane:  &'a mut NcPlane,
    hdg_plane:  &'a mut NcPlane,
    body_plane: &'a mut NcPlane,

    upvotes: u32,
    heading: String,
    content: String,
    username: String,
    subreddit_name: String,
    comments: u32,
    body: String
}

impl<'a> SubListPost<'a> {
    fn set_contents(&mut self, data: PostData) {
        self.upvotes = data.upvotes;
        self.heading = data.heading.to_string();
        self.content = data.content.to_string();
        self.username = data.username.to_string();
        self.subreddit_name = data.subreddit_name.to_string();
        self.comments = data.comments;
        self.body = data.body.to_string();
    }

    fn draw_header(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        let upvoted_channel = NcChannels::from_rgb(
            tui_prefs.theme.post_upvoted_fg.to_nc_rgb(),
            tui_prefs.theme.post_upvoted_bg.to_nc_rgb()
        );

        const UPVOTE_COUNT_DECIMAL_PRECISION: u32 = 7;
        const MAX_USERNAME_LEN: u32 = 16;
        const COMMENT_COUNT_DECIMAL_PRECISION: u32 = 8;

        let mut pos = 0;
        self.hdr_plane.putstr_yx_stained(0, pos, &self.upvotes.to_string())?;

        pos = UPVOTE_COUNT_DECIMAL_PRECISION + 1;
        self.hdr_plane.putstr_yx(Some(0), Some(pos), &self.username)?;

        pos = pos + MAX_USERNAME_LEN + 1;
        self.hdr_plane.putstr_yx(Some(0), Some(pos), &self.subreddit_name)?;

        pos = self.plane.dim_x() - COMMENT_COUNT_DECIMAL_PRECISION + 1;
        self.hdr_plane.putstr_yx(Some(0), Some(pos), &self.comments.to_string())?;

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
        self.hdg_plane.erase();
        self.hdg_plane.puttext(0, NcAlign::Left, &self.heading)?;
        Ok(())
    }

    // TODO: Safeguard against text overflow since the App crashes.
    fn draw_body(&mut self) -> Result<()> {
        self.body_plane.erase();
        self.body_plane.puttext(0, NcAlign::Left, &self.body)?;
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

        let body_plane = new_child_plane!(plane, 0, 2, dim_x, 3);
        body_plane.set_base(
            " ",
            0,
            NcChannels::from_rgb(
                tui_prefs.theme.post_body_fg.to_nc_rgb(),
                tui_prefs.theme.post_body_bg.to_nc_rgb()
            ))?;
        body_plane.set_fg_rgb(tui_prefs.theme.post_body_fg.to_nc_rgb());
        body_plane.set_bg_rgb(tui_prefs.theme.post_body_bg.to_nc_rgb());


        let hdr_plane = new_child_plane!(plane, 0, 0, dim_x, 1);
        hdr_plane.set_base(
            " ",
            0,
            NcChannels::from_rgb(
                tui_prefs.theme.post_header_fg.to_nc_rgb(),
                tui_prefs.theme.post_header_bg.to_nc_rgb()
            ))?;
        hdr_plane.set_fg_rgb(tui_prefs.theme.post_header_fg.to_nc_rgb());
        hdr_plane.set_bg_rgb(tui_prefs.theme.post_header_bg.to_nc_rgb());

        let hdg_plane = new_child_plane!(plane, 0, 1, dim_x, 1);
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

                hdr_plane,
                hdg_plane,
                body_plane,

                heading: "".to_string(),
                content: "".to_string(),
                upvotes: 0,
                username: "".to_string(),
                subreddit_name: "".to_string(),
                comments: 0,
                body: "".to_string()

        })
    }

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        log_err_desc_ret!(self.draw_header(tui_prefs), "Failed to draw header.")?;
        log_err_desc_ret!(self.draw_heading(), "Failed to draw heading.")?;
        log_err_desc_ret!(self.draw_body(), "Failed to draw body.")
    }
}

impl<'a> Group for SubListPost<'a> {
    fn move_rel_xy(&mut self, x_diff: i32, y_diff: i32) -> Result<()> {
        self.plane.move_rel(y_diff, x_diff)?;
        self.hdr_plane.move_rel(y_diff, x_diff)?;
        self.hdg_plane.move_rel(y_diff, x_diff)?;
        self.body_plane.move_rel(y_diff, x_diff)?;
        Ok(())
    }
}

// -----------------------------------------------------------------------------------------------------------
// Page for displaying subreddit listing.
// -----------------------------------------------------------------------------------------------------------
pub struct SubListPage<'a> {
    pub plane: &'a mut NcPlane,
    posts: Vec<SubListPost<'a>>,

    visible: bool, // Whether the page should be visible. If not, set if off right of visible area.

    scrolled: u32, // Lines scrolled down, 0 initially.
    content_len: u32
}

impl<'a> SubListPage<'a> {
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

        let page = Self { 
            plane,
            posts: vec![],
            visible: true,
            scrolled: 0,
            content_len: 0
        };
        
        Ok(page)
    }

    fn draw(&mut self, _tui_prefs: &TuiPrefs) -> Result<()> {
        Ok(())
    }
}

impl<'a> Page for SubListPage<'a> {
    fn set_visibility(&mut self, visible: bool) -> Result<()> {
        if visible != self.visible {
            if visible {
                self.move_rel_xy(self.plane.dim_x() as i32, 0)?;
            } else {
                self.move_rel_xy(- (self.plane.dim_x() as i32), 0)?;
            }
            self.visible = visible;
        }
        Ok(())
    }

    fn scroll_down(&mut self) -> Result<()> {
        if self.scrolled + self.plane.dim_y() >= self.content_len - 1 {
            bail!("Bottom reached, cannot scroll down more.");
        }
        self.scrolled += 2;
        self.move_rel_xy(0, -2)
    }

    fn add_post(&mut self, tui_prefs: &TuiPrefs, data: PostData) -> Result<()> {
        let mut post = SubListPost::new(
                tui_prefs,
                self.plane,
                0,
                (self.posts.len() * 5) as i32,
                self.plane.dim_x(),
                self.plane.dim_y(),
            )?;
        post.set_contents(data);
        self.posts.push(post);
        self.content_len += 5;
        Ok(())
    }
    fn scroll_up(&mut self) -> Result<()> {
        if self.scrolled == 0 {
            bail!("Top reached, cannot scroll up more.");
        }
        self.scrolled -= 2;
        self.move_rel_xy(0, 2)
    }

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

impl Group for SubListPage<'_> {
    fn move_rel_xy(&mut self, x_diff: i32, y_diff: i32) -> Result<()> {
        self.plane.move_rel(y_diff, x_diff)?;
        for post in self.posts.iter_mut() {
            post.move_rel_xy(x_diff, y_diff)?;
        }
        Ok(())
    }
}
