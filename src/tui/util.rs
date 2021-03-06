use anyhow::{ anyhow, Result };
use log::error;
use libnotcurses_sys::{ NcPlane, NcRgb };

use crate::jobs::TuiPrefsDes;

// TODO: Add test to check if we're validating all fields and formats.
pub fn val_tui_prefs_des(tui_prefs_des: &TuiPrefsDes) -> bool {
    // Check color format
    let val_color_fmt = |s: &str| -> bool {
        let mut res = true;
        res = res && s.len() == 7;
        let mut chars = s.chars();
        res = res && Some('#') == chars.next();
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_hexdigit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_hexdigit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_hexdigit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_hexdigit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_hexdigit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_hexdigit() );
        res
    };
        
    let mut res = true;

    // Check theme
    {
        let theme = &tui_prefs_des.theme;
        let mut temp_bool: bool;

        temp_bool = val_color_fmt(&theme.highlight_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "highlight-fg", theme.highlight_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.highlight_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "highlight-bg", theme.highlight_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.page_bar_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "page-bar-fg", theme.page_bar_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.page_bar_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "page-bar-bg", theme.page_bar_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.page_bar_current_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "page-bar-current-bg", theme.page_bar_current_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_header_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-header-fg", theme.post_header_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_header_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-header-bg", theme.post_header_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_upvoted_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-upvoted-fg", theme.post_upvoted_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_upvoted_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-upvoted-bg", theme.post_upvoted_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_heading_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-heading-fg", theme.post_heading_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_heading_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-heading-bg", theme.post_heading_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_body_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-body-fg", theme.post_body_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.post_body_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-body-bg", theme.post_body_bg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.cmd_plt_fg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-body-bg", theme.cmd_plt_fg);
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.cmd_plt_bg);
        if !temp_bool {
            error!("Wrong color format for {} {} - {}", "theme", "post-body-bg", theme.cmd_plt_bg);
        }
        res = res && temp_bool;
    }

    res
}


#[derive(Clone)]
#[cfg_attr(test, derive(PartialEq, Debug))]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn get_color_from_str(color_str: &str) -> Option<Color> {
        let mut r: u8;
        let mut g: u8;
        let mut b: u8;

        let mut chars = color_str.chars();
        chars.next(); //Skipping through #

        r = chars.next()?.to_digit(16)?.to_le_bytes()[0];
        r = r*16 + chars.next()?.to_digit(16)?.to_le_bytes()[0];

        g = chars.next()?.to_digit(16)?.to_le_bytes()[0];
        g = g*16 + chars.next()?.to_digit(16)?.to_le_bytes()[0];

        b = chars.next()?.to_digit(16)?.to_le_bytes()[0];
        b = b*16 + chars.next()?.to_digit(16)?.to_le_bytes()[0];

        Some(Color {
            r,
            b,
            g
        })
    }    
    
    pub fn to_nc_rgb(&self) -> NcRgb {
       let mut rgb: u32 = 0; 
       rgb = rgb ^ ((self.r as u32) << 16);
       rgb = rgb ^ ((self.g as u32) <<  8);
       rgb = rgb ^ ((self.b as u32)      );
       NcRgb(rgb)
    }
}

// Interface preferences.
pub struct InterfacePrefs {
    pub mouse_events_enable: bool
}

// Theme.
pub struct Theme {
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub page_bar_fg: Color,
    pub page_bar_bg: Color,
    pub page_bar_current_bg: Color,
    pub post_header_fg: Color,
    pub post_header_bg: Color,
    pub post_upvoted_fg: Color,
    pub post_upvoted_bg: Color,
    pub post_heading_fg: Color,
    pub post_heading_bg: Color,
    pub post_body_fg: Color,
    pub post_body_bg: Color,
    pub cmd_plt_fg: Color,
    pub cmd_plt_bg: Color
}

// TUI preferences.
pub struct TuiPrefs {
    pub interface: InterfacePrefs,
    pub theme: Theme
}

impl TuiPrefs {
    pub fn gen_tui_prefs(tui_prefs_des: &TuiPrefsDes) -> Result<TuiPrefs> {
        Ok(
            TuiPrefs {
                interface: InterfacePrefs {
                    mouse_events_enable: tui_prefs_des.interface.mouse_events_enable
                },
                theme: Theme {
                    highlight_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.highlight_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    highlight_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.highlight_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    page_bar_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.page_bar_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    page_bar_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.page_bar_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    page_bar_current_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.page_bar_current_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_header_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_header_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_header_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_header_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_upvoted_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_upvoted_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_upvoted_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_upvoted_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_heading_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_heading_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_heading_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_heading_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_body_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_body_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    post_body_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.post_body_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    cmd_plt_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.cmd_plt_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    cmd_plt_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.cmd_plt_bg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                }
            }
        )
    }
}

// -----------------------------------------------------------------------------------------------------------
// Widget trait
// * Widgets are functional components that make the App.
// -----------------------------------------------------------------------------------------------------------
pub trait Widget: Sized {
    fn new(tui_prefs: &TuiPrefs,
            plane: &mut NcPlane,
            x: i32,
            y: i32,
            dim_x: u32,
            dim_y: u32
            ) -> Result<Self>;
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;
}

// -----------------------------------------------------------------------------------------------------------
// Group trait
// * Groups children widgets with a parent widget.
// * Useful for applying transformations to entire group like moving the group together.
// -----------------------------------------------------------------------------------------------------------
pub trait Group {
    fn move_rel_xy(&mut self, x_diff: i32, y_diff: i32) -> Result<()>;
}

macro_rules! new_child_plane {
    { $parent_plane: expr, $x: expr, $y: expr, $dim_x: expr, $dim_y: expr} => {
        NcPlane::new_child($parent_plane, &NcPlaneOptions::new($y, $x, $dim_y, $dim_x))?
    }
}
pub(super) use new_child_plane;

// Data structures

pub struct PostData<'a> {
    pub upvotes: u32,
    pub heading: &'a str,
    pub content: &'a str,
    pub username: &'a str,
    pub subreddit_name: &'a str,
    pub comments: u32,
    pub body: &'a str
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_gen_color_from_str() {
        assert_eq!(Some(Color {
            r: 255u8,
            g: 255u8,
            b: 255u8
        }), Color::get_color_from_str("#ffffff"));

        assert_eq!(Some(Color {
            r: 166u8,
            g: 183u8,
            b: 200u8
        }), Color::get_color_from_str("#a6b7c8"));

        assert_eq!(None, Color::get_color_from_str("#g6b7c6"));
    }
}
