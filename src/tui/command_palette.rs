use anyhow::{ anyhow, Result };
use log::error;
use libnotcurses_sys::{
    c_api::{ ncreader, ncreader_contents, ncreader_clear, ncreader_destroy, ncreader_offer_input },
    NcChannel,
    NcChannels,
    NcInput,
    NcKey,
    NcPlane,
    NcPlaneOptions,
    NcReceived,
    widgets::NcReaderOptions
};
use std::ffi::CStr;

use super::{ TuiPrefs, util::new_child_plane, util::Widget };
use crate::tui::AppRes;
use crate::tools::log_err_ret;

// -----------------------------------------------------------------------------------------------------------
// Command palette widget.
// -----------------------------------------------------------------------------------------------------------
pub struct CmdPalette<'a> {
    pub plane: &'a mut NcPlane,
    pub reader: &'a mut ncreader,
}

impl<'a> CmdPalette<'a> {
    // Add input.
    pub fn input(&mut self, ncin: NcInput) -> Result<AppRes> {
        if unsafe { ncreader_offer_input(self.reader, &ncin) } {
            match self.contents() {
                Ok(s) => {
                    if s.len() == 1 && (0, 0) == self.plane.cursor_yx() {
                        return Ok(AppRes::CmdModeQuit);
                    }    
                },
                Err(e) => {
                    error!("Failed to get command palette contents: {}", e);
                    return Err(e);
                }
            }
            return Ok(AppRes::CmdModeCont)
        }
        return log_err_ret!(Err(anyhow!("Unable to input to command palette: {:?}", ncin)))
    }

    // Get contents of command palette.
    pub fn contents(&mut self) -> Result<String> {
        Ok((unsafe { CStr::from_ptr(ncreader_contents(self.reader)) }).to_str()?.to_string())
    }

    pub fn clear_contents(&mut self) {
        unsafe { ncreader_clear(self.reader) };
    }

    // Destroy reader nc widget. Required for graceful termination of application.
    pub fn destroy_reader(&mut self) {
        unsafe { ncreader_destroy(self.reader, std::ptr::null::<*mut *mut i8>() as *mut *mut i8) }
    }

}

// Validate input for command palette.
pub fn cmd_plt_val_input(ncr: &NcReceived) -> bool {
    match ncr {
        NcReceived::Char(_) => true,
        NcReceived::Event(NcKey::Left) => true,
        NcReceived::Event(NcKey::Right) => true,
        NcReceived::Event(NcKey::Enter) => true,
        NcReceived::Event(NcKey::Backspace) => true,
        _ => false
    }
}

impl<'a> Widget for CmdPalette<'a> {
    fn new(_: &TuiPrefs,
            parent_plane: &mut NcPlane,
            x: i32,
            y: i32,
            dim_x: u32,
            dim_y: u32
          ) -> Result<Self> {

        let plane = new_child_plane!(parent_plane, x, y, dim_x, dim_y);

        let mut header_bg_channel = NcChannel::new();
        header_bg_channel.set_r(100);
        header_bg_channel.set_g(100);
        header_bg_channel.set_b(100);
        let mut header_fg_channel = NcChannel::new();
        header_fg_channel.set_r(0);
        header_fg_channel.set_g(200);
        header_fg_channel.set_b(0);
        let header_combined_channel = NcChannels::combine(header_fg_channel, header_bg_channel);

        plane.set_channels(header_combined_channel);

        let reader = ncreader::with_options(
                plane,
                &NcReaderOptions {
                    tchannels: header_combined_channel.0,
                    tattrword: 0,
                    flags: (NcReaderOptions::CURSOR | NcReaderOptions::HORSCROLL) as u64
                }
            )?; 

        Ok(Self {
            plane,
            reader
        })
    }

    fn draw(&mut self, _tui_prefs: &TuiPrefs) -> Result<()> {
        Ok(())
    }
}
