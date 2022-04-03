use libnotcurses_sys::NcInput;

pub enum Message {
    InitTUI,
    AppQuit,
    CmdInput(NcInput)
}
