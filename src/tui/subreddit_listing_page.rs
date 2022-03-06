use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    NcPlane,
    NcPlaneOptions,
    NcAlign,
};

use super::{ Color, page::{ Page }, TuiPrefs, TuiWidget, util::new_child_plane };

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
    pub fn new<'b>(tui_prefs: &TuiPrefs,
                    plane: &'b mut NcPlane,
                    data: SubListPostData<'b>
                   ) -> Result<SubListPost<'b>> {
        Ok(SubListPost {
                plane: plane,
                data: data
        })
    }
    // Draw the plane (not render) using the properties.
    pub fn draw(&mut self) -> Result<()> {
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
    pub fn new<'b>(tui_prefs: &TuiPrefs,
                   plane: &'b mut NcPlane,
                   ) -> Result<Box<SubListPage<'b>>> {
        Ok(Box::new(SubListPage::<'b>{ 
            plane: plane,
            data: SubListPageData {
                name: "Cyberpunk"
            },
            posts: vec![]
        }))
    }

    pub fn add_post(&mut self, tui_prefs: &TuiPrefs, data: SubListPostData<'a>) -> Result<()> {
        self.posts.push(SubListPost::new(
                tui_prefs,
                new_child_plane!(self.plane, 0, 0, 100, 100),
                data
            )?);
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
