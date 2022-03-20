use anyhow::{ anyhow, Result };
use log::{ error, info };
use libnotcurses_sys::NcPlane;

use crate::jobs::TuiPrefsDes;

// TODO: Add test to check if we're validating all fields and formats.
pub fn val_tui_prefs_des(tui_prefs_des: &TuiPrefsDes) -> bool {

    info!("Validing TUI Prefs.");

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
}

pub struct Theme {
    pub highlight_fg: Color,
    pub highlight_bg: Color,
    pub post_header_fg: Color,
    pub post_header_bg: Color,
    pub post_upvoted_fg: Color,
    pub post_upvoted_bg: Color,
    pub post_heading_fg: Color,
    pub post_heading_bg: Color
}

pub struct TuiPrefs {
    pub theme: Theme
}

impl TuiPrefs {
    pub fn gen_tui_prefs(tui_prefs_des: &TuiPrefsDes) -> Result<TuiPrefs> {
        info!("Generating TUI Prefs.");
        Ok(
            TuiPrefs {
                theme: Theme {
                    highlight_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.highlight_fg) 
                        { color } else { return Err(anyhow!("Invalid color format.")); },
                    highlight_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.highlight_bg) 
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
                        { color } else { return Err(anyhow!("Invalid color format.")); }
                }
            }
        )
    }
}

// Drop trait because libnotcurses_sys doesn't call destructor methods.
pub trait Widget: Sized + Drop {
    fn new(tui_prefs: &TuiPrefs,
            plane: &mut NcPlane,
            x: i32,
            y: i32,
            dim_x: u32,
            dim_y: u32
            ) -> Result<Self>;
    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()>;
}

macro_rules! new_child_plane {
    { $parent_plane: expr, $x: expr, $y: expr, $dim_x: expr, $dim_y: expr} => {
        NcPlane::new_child($parent_plane, &NcPlaneOptions::new($x, $y, $dim_x, $dim_y))?
    }
}
pub(super) use new_child_plane;

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
