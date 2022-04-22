// use libnotcurses_sys::NcInput;
// use tokio::sync::{ oneshot::Sender as OneshotSender, mpsc::Sender as MpscSender };
// use crate::input::input_message::InputMessage;
//
// // -----------------------------------------------------------------------------------------------------------
// // Message structs used to communicate to manager by events.
// // -----------------------------------------------------------------------------------------------------------
// #[derive(Debug)]
// pub enum Message {
//     // Initialize TUI.
//     InitTUI,
//
//     // Quit app. Send ACK.
//     AppQuit(OneshotSender<InputMessage>),
//
//     // Enter Cmd Mode
//     CmdEnter,
//
//     // Exit Cmd Mode
//     CmdExit,
//
//     // Execute command
//     CmdExec,
//
//     // New input for command palette widget.
//     CmdInput(NcInput, OneshotSender<InputMessage>),
// }
