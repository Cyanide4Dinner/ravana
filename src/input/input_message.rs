// -----------------------------------------------------------------------------------------------------------
// Messages sent to input (listener) of oneshot channel. 
// -----------------------------------------------------------------------------------------------------------
#[derive(Debug)]
pub enum InputMessage {
    AppQuit,
    ContinueCmdMode,
    EndCmdMode,
}
