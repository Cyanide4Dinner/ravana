use anyhow::{ anyhow, Context, Result };
use libnotcurses_sys::{
    NcPlane,
    NcPlaneOptions,
    NcAlign,
};

pub struct SubListPostProps {
    pub dim_y: u32,
    pub dim_x: u32
}

pub struct SubListPost<'a> {
    plane: &'a mut NcPlane,
    heading: &'a str,
    upvotes: u32,
    username: &'a str,
    content: &'a str,
}

impl<'a> SubListPost<'a> {
    pub fn new<'b>(sl_page_plane: &'b mut NcPlane, props: SubListPostProps) -> Result<SubListPost<'b>> {
        Ok(SubListPost {
            plane:  NcPlane::new_child(sl_page_plane, &NcPlaneOptions::new(0, 0, props.dim_y, props.dim_x))?,
            heading: "",
            upvotes: 0,
            content: "",
            username: "",
        })
    }
    // Draw the plane (not render) using the properties.
    pub fn draw(&mut self) {
       self.plane.set_bg_rgb8(226u8, 36u8, 36u8); 
       self.plane.puttext(0, NcAlign::Left, "dafjaldfjald fjalsdf ajsdflkjasdf lajsdflka dfjlakdjf");
       self.plane.puttext(1, NcAlign::Left, "dkfjadlfad faldf adfa \n fafdj");
    }
}

pub struct SubListData<'a> {
    posts: Vec<SubListPost<'a>>
}

pub struct SubListPageProps {
    pub dim_x: u32,
    pub dim_y: u32
}

pub struct SubListPage<'a> {
    base_plane: &'a mut NcPlane,
    subreddit_name: &'a str,
    data: SubListData<'a>
}

impl<'a> SubListPage<'a> {
    pub fn new<'b>(app_page: &'b mut NcPlane, sl_page_props: SubListPageProps) -> Result<Box<SubListPage<'a>>> {
        let base_plane = NcPlane::new_child(app_page, &NcPlaneOptions::new(0, 0, sl_page_props.dim_y, sl_page_props.dim_x))?;

        let mut dummy_listing_post = SubListPost { 
            heading: "Try Heading", 
            content: "Try Content", 
            upvotes: 0,
            username: "hi",
            plane: NcPlane::new_child(base_plane, &NcPlaneOptions::new(0, 0, sl_page_props.dim_y/4, sl_page_props.dim_x))? };
        dummy_listing_post.draw();
        let dummmy_listing_data = SubListData { posts: vec!{dummy_listing_post} }; 
        
        Ok(Box::new(SubListPage { base_plane: base_plane, subreddit_name: "Cyberpunk", data: dummmy_listing_data }))
    }
}
