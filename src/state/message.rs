use libnotcurses_sys::NcInput;
use tokio::sync::oneshot::Sender;

#[derive(Debug)]
pub enum Message {
    InitTUI,
    AppQuit(Sender<bool>),
    CmdInput(NcInput)
}
