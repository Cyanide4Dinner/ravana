use anyhow::{ anyhow, Result };
use log::{ error, info, warn };

use crate::jobs::{ TuiPrefsDes };

// TODO: Add test to check if we're validating all fields and formats.
pub fn val_tui_prefs_des(tui_prefs_des: &TuiPrefsDes) -> bool {

    info!("Validing TUI Prefs.");

    // Check color format
    let val_color_fmt = |s: &str| -> bool {
        let res = true;
        res = res && s.len() == 7;
        let mut chars = s.chars();
        res = res && Some('#') == chars.next();
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_digit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_digit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_digit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_digit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_digit() );
        res = res && chars.next().map_or(false, |c: char| c.is_ascii_digit() );
        res
    };
        
    let res = true;

    // Check theme
    {
        let theme = tui_prefs_des.theme;
        let mut temp_bool: bool;

        temp_bool = val_color_fmt(&theme.highlight_fg);
        if !temp_bool {
            error!("Wrong color format for {} {}", "theme", "highlight_fg");
        }
        res = res && temp_bool;

        temp_bool = val_color_fmt(&theme.highlight_bg);
        if !temp_bool {
            error!("Wrong color format for {} {}", "theme", "highlight_bg");
        }
        res = res && temp_bool;
    }

    res
}


pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Color {
    fn get_color_from_str(color_str: &str) -> Option<Color> {
        let r: u8;
        let g: u8;
        let b: u8;

        let chars = color_str.chars();

        r = chars.next()?.to_digit(16)?.to_le_bytes()[0];
        r = r*16 + chars.next()?.to_digit(16)?.to_le_bytes()[0];

        g = chars.next()?.to_digit(16)?.to_le_bytes()[0];
        g = g*16 + chars.next()?.to_digit(16)?.to_le_bytes()[0];

        b = chars.next()?.to_digit(16)?.to_le_bytes()[0];
        b = b*16 + chars.next()?.to_digit(16)?.to_le_bytes()[0];

        Some(Color {
            r: r,
            g: g,
            b: b
        })
    }    
}

pub struct Theme {
    highlight_fg: Color,
    highlight_bg: Color
}

pub struct TuiPrefs {
    theme: Theme
}

impl TuiPrefs {
    pub fn gen_tui_prefs(tui_prefs_des: &TuiPrefsDes) -> Result<TuiPrefs> {
        Ok(
            TuiPrefs {
                theme: Theme {
                    highlight_fg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.highlight_fg) 
                        { color } else { return anyhow!("Invalid color format.") },
                    highlight_bg: if let Some(color) = Color::get_color_from_str(&tui_prefs_des.theme.highlight_bg) 
                        { color } else { return anyhow!("Invalid color format.") }
                }
            }
        )
    }
}
