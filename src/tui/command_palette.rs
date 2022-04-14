use anyhow::{ anyhow, bail, Result };
use log::{ debug, error, info };
use libnotcurses_sys::{
    c_api::{ ncreader, ncreader_contents, ncreader_destroy, ncreader_offer_input },
    NcChannel,
    NcChannels,
    NcInput,
    NcKey,
    NcPlane,
    NcPlaneOptions,
    NcReceived,
    widgets::NcReaderOptions
};
use tokio::sync::oneshot;

use super::{ TuiPrefs, util::new_child_plane, Widget };
use crate::input::input_message::InputMessage;

// -----------------------------------------------------------------------------------------------------------
// Command palette widget.
// -----------------------------------------------------------------------------------------------------------
pub struct CmdPalette<'a> {
    pub plane: &'a mut NcPlane,
    reader: &'a mut ncreader
}

impl<'a> CmdPalette<'a> {
    // Add input.
    pub fn input(&mut self, ncin: NcInput, oneshot_tx: oneshot::Sender<InputMessage>) -> Result<()> {
        if unsafe { ncreader_offer_input(self.reader, &ncin) } {
            if let Err(e) = oneshot_tx.send(InputMessage::ContinueCmdMode) {
                error!("Error sending oneshot_tx: {:?}", e);
            };
            Ok(())
        } else {
            bail!("Unable to input to command palette: {:?}", ncin)
        }
    }

    // Validate input.
    pub fn val_input(ncr: &NcReceived) -> bool {
        match ncr {
            NcReceived::Char(_) => true,
            NcReceived::Event(NcKey::Left) => true,
            NcReceived::Event(NcKey::Right) => true,
            _ => false
        }
    }

    // Get contents of command palette.
    // pub fn contents(&mut self) -> Result<String> {
    // }

    // Destroy reader nc widget. Required for graceful termination of application.
    pub fn destroy_reader(&mut self) {
        debug!("Destroying CmdPalette.");
        unsafe { ncreader_destroy(self.reader, std::ptr::null::<*mut *mut i8>() as *mut *mut i8) }
    }

}

impl<'a> Widget for CmdPalette<'a> {
    fn new(tui_prefs: &TuiPrefs,
            parent_plane: &mut NcPlane,
            x: i32,
            y: i32,
            dim_x: u32,
            dim_y: u32
          ) -> Result<Self> {
        debug!("Creating new command palette.");

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

    fn draw(&mut self, tui_prefs: &TuiPrefs) -> Result<()> {
        Ok(())
    }
}