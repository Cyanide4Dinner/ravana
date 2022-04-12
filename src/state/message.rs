use libnotcurses_sys::NcInput;
use tokio::sync::oneshot::Sender;

// -----------------------------------------------------------------------------------------------------------
// Message structs used to communicate to manager by events.
// -----------------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum Message {
    // Initialize TUI.
    InitTUI,

    // Quit app. Send ACK.
    AppQuit(Sender<bool>),

    // New input for command palette widget.
    CmdInput(NcInput)
}
