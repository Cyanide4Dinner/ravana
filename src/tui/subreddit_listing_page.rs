use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    NcPlane,
    NcPlaneOptions,
    NcAlign,
};

use super::{ Color, page::{ Page }, TuiPrefs, Widget, util::new_child_plane };

pub struct SubListPostData<'a> {
    pub upvotes: u32,
    pub heading: &'a str,
    pub content: &'a str,
    pub username: &'a str
}

pub struct SubListPost<'a> {
    plane: &'a mut NcPlane,
    data: SubListPostData<'a>
}

impl<'a> SubListPost<'a> {
    // Draw the plane (not render) using the properties.
}

impl<'a> Widget for SubListPost<'a> {
    fn new(tui_prefs: &TuiPrefs,
                    plane: &mut NcPlane,
                    x: i32,
                    y: i32,
                    dim_x: u32,
                    dim_y: u32
                   ) -> Result<Self> {
        Ok(Self {
                plane: new_child_plane!(plane, x, y, dim_x, dim_y),
                data: SubListPostData {
                    heading: "",
                    content: "",
                    upvotes: 0,
                    username: ""
                }
        })
    }

    fn draw(&mut self) -> Result<()> {
       self.plane.puttext(0, NcAlign::Left, "dafjaldfjald fjalsdf ajsdflkjasdf lajsdflka dfjlakdjf")?;
       self.plane.puttext(1, NcAlign::Left, "dkfjadlfad faldf adfa \n fafdj")?;
       Ok(())
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
                100,
                100
            )?);
        Ok(())
    }
}

impl<'a> Widget for SubListPage<'a> {
    fn new(tui_prefs: &TuiPrefs,
                   plane: &mut NcPlane,
                   x: i32,
                   y: i32,
                   dim_x: u32,
                   dim_y: u32
                   ) -> Result<Self> {
        Ok(Self { 
            plane: new_child_plane!(plane, x, y, dim_x, dim_y),
            data: SubListPageData {
                name: "Cyberpunk"
            },
            posts: vec![]
        })
    }

    fn draw(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Page for SubListPage<'_> {
    fn draw(&mut self) -> Result<()> {
        for post in self.posts.iter_mut() {
            post.draw().context("Failed to render post.")?;
        }
        Ok(())
    }

    fn fetch(&mut self) -> Result<()> {
        Ok(())
    }
}
