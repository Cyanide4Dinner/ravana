use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    NcAlign,
    NcChannel,
    NcChannelApi,
    NcChannels,
    NcChannelsApi,
    NcCell,
    NcPlane,
    NcPlaneOptions,
    NcStyle,
};
use log::{ error, info, warn };

use super::{ Color, page::{ Page }, TuiPrefs, Widget, util::new_child_plane };

pub struct SubListPostData<'a> {
    pub upvotes: u32,
    pub heading: &'a str,
    pub content: &'a str,
    pub username: &'a str,
    pub subreddit_name: &'a str,
    pub comments: u32
}

pub struct SubListPost<'a> {
    plane: &'a mut NcPlane,
    data: SubListPostData<'a>
}

impl<'a> SubListPost<'a> {
    fn draw_header(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        let header_bg_channel = NcChannel::from_rgb8(
                tui_prefs.theme.post_header_bg.r,
                tui_prefs.theme.post_header_bg.b,
                tui_prefs.theme.post_header_bg.g,
            );
        let header_fg_channel = NcChannel::from_rgb8(
                tui_prefs.theme.post_header_fg.r,
                tui_prefs.theme.post_header_fg.b,
                tui_prefs.theme.post_header_fg.g,
            );
        let upvoted_channel = NcChannels::from_rgb8(
                tui_prefs.theme.post_upvoted_fg.r, 
                tui_prefs.theme.post_upvoted_fg.b, 
                tui_prefs.theme.post_upvoted_fg.g, 
                tui_prefs.theme.post_upvoted_bg.r, 
                tui_prefs.theme.post_upvoted_bg.b, 
                tui_prefs.theme.post_upvoted_bg.g 
            );

        let header_combined_channel = NcChannels::combine(header_fg_channel, header_bg_channel);

        const UPVOTE_COUNT_DECIMAL_PRECISION: u32 = 7;
        const MAX_USERNAME_LEN: u32 = 16;
        const COMMENT_COUNT_DECIMAL_PRECISION: u32 = 8;

        // Fill space as character to get color on while line.
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
}

impl<'a> Drop for SubListPost<'a> {
    fn drop(&mut self) {
        if let Err(err) = self.plane.destroy() {
            error!("Error dropping SubListPost plane: {}", err);
        }
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
                plane: plane,
                data: SubListPostData {
                    heading: "",
                    content: "",
                    upvotes: 18901,
                    username: "AyeDeeKay",
                    subreddit_name: "Rust",
                    comments: 17
                }
        })
    }

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        self.draw_header(tui_prefs)
    }
}

pub struct SubListPageData<'a> {
    name: &'a str
}

pub struct SubListPage<'a> {
    plane: &'a mut NcPlane,
    data: SubListPageData<'a>,
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

impl<'a> Drop for SubListPage<'a> {
    fn drop(&mut self) {
        for post in self.posts.iter_mut() {
            drop(post)
        }
        if let Err(err) = self.plane.destroy() {
            error!("Error dropping SubListPage plane: {}", err);
        }
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

        plane.set_fg_rgb8(
            tui_prefs.theme.highlight_fg.r,
            tui_prefs.theme.highlight_fg.g,
            tui_prefs.theme.highlight_fg.b,
        );
        plane.set_bg_rgb8(
            tui_prefs.theme.highlight_bg.r,
            tui_prefs.theme.highlight_bg.g,
            tui_prefs.theme.highlight_bg.b,
        );
        
        Ok(Self { 
            plane: plane,
            data: SubListPageData {
                name: "Cyberpunk"
            },
            posts: vec![]
        })
    }

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
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
