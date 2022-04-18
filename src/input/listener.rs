use anyhow::{ Context, Result };
use libnotcurses_sys::{
    c_api::notcurses_inputready_fd,
    Nc,
    NcInput,
    NcKey,
    NcReceived,
}; 
use log::{ debug, error, warn };
use nix::poll::{ poll, PollFd, PollFlags };
use std::{ collections::HashMap, sync::{ Arc, Mutex } };
use tokio::sync::{ oneshot, mpsc::Sender };

use crate::events::app_events::init_tui;
use crate::tui::CmdPalette;
use crate::state::Message;
use super::command_to_event::exec_cmd;
use super::util::key_bindings::{ 
    create_key_bindings_trie,
    Key,
    KeyBindingsTrie,
    KeyCombination,
};
use crate::input::input_message::InputMessage;

// -----------------------------------------------------------------------------------------------------------
// * Generate key-bindings trie.
// * Initialize input listener.
// -----------------------------------------------------------------------------------------------------------
pub async fn init(nc: Arc<Mutex<&mut Nc>>,
                  kb: HashMap<String, String>,
                  mpsc_send: Sender<Message>,
                  )
        -> Result<()> {
    debug!("Init input listener.");

    let kbt = create_key_bindings_trie(&kb).context("Error parsing key-bindings.")?;

    init_tui(mpsc_send.clone()).await?; 

    listen(nc, kbt, mpsc_send.clone()).await?;
    Ok(())
}

//TODO: Create tests for event loop checking and ensuring.

// -----------------------------------------------------------------------------------------------------------
// * Poll on inputready_fd.
// * Buffer inputs until key-bindings match.
// * Manage COMMAND INPUT MODE, toggling it based on key-bindings.
// -----------------------------------------------------------------------------------------------------------
async fn listen(nc: Arc<Mutex<&mut Nc>>,
                kbt: KeyBindingsTrie,
                mpsc_send: Sender<Message>
                ) -> Result<()> {
    debug!("Begin input listening loop.");
    let mut buffer: KeyCombination = KeyCombination::new();
    let mut input_details = NcInput::new_empty();

    // COMMAND INPUT MODE
    let mut cmd_input: bool = false;

    let mut nc_lock = nc.lock().unwrap();
    let input_fd = PollFd::new(
        unsafe { notcurses_inputready_fd(*nc_lock as &mut Nc as *mut Nc) },
        PollFlags::POLLIN);
    drop(nc_lock);

    loop {
        // Oneshot channel to receive response for input listener.
        let (oneshot_tx, oneshot_rx) = oneshot::channel::<InputMessage>();

        if let Ok(_) = poll(&mut [input_fd], -1) {
            nc_lock = nc.lock().unwrap();
            let recorded_input = nc_lock.get_nblock(Some(&mut input_details))?;
            drop(nc_lock);

            // -----------------------------------------------------------------------------------------------
            // COMMAND INPUT MODE - true
            // -----------------------------------------------------------------------------------------------
            if cmd_input {
                match recorded_input {
                    // Execute command.
                    NcReceived::Event(NcKey::Enter) => {
                        debug!("Preparing to execute command.");
                        cmd_input = false;
                        if let Err(e) = mpsc_send.send(Message::CmdExec).await {
                            error!("Error sending CmdExec mpsc_send message: {}", e);
                        };
                        continue;
                    },

                    // Escape command mode.
                    NcReceived::Event(NcKey::Esc) => {
                        debug!("Escaping command mode.");
                        cmd_input = false;
                        if let Err(e) = mpsc_send.send(Message::CmdExit).await {
                            error!("Error sending CmdExit mpsc_send message: {}", e);
                        };
                        continue;
                    },

                    _ => {
                        // Validate if input recieved is compatible.
                        if CmdPalette::val_input(&recorded_input) { 
                            if let Err(e) = mpsc_send.send(Message::CmdInput(input_details.clone(),
                                                    oneshot_tx)).await {
                                    error!("Error sending CmdInput mpsc_send message: {}", e);
                            };
                        }
                    }
                }

                // Wait for confirmation before continuing.
                if let Ok(input_msg) = oneshot_rx.await {
                    match input_msg {
                        InputMessage::ContinueCmdMode => { continue; },
                        InputMessage::EndCmdMode => { cmd_input = false; continue; },
                        _ => { 
                            error!("Wrong message received by listener in CmdMode: {:?}", input_msg); 
                            cmd_input = false;
                        }
                    }
                } else {
                    error!("Error receiving from oneshot channel in listener.");
                }
            } 
            // -----------------------------------------------------------------------------------------------
            // COMMAND INPUT MODE - false
            // -----------------------------------------------------------------------------------------------
            else { // COMMAND INPUT MODE - false
                if let NcReceived::Char(':') = recorded_input {
                    // Switch to COMMAND INPUT MODE.
                    debug!("COMMAND INPUT MODE - ON");
                    if let Err(e) = mpsc_send.send(Message::CmdEnter).await {
                            error!("Error sending CmdEnter mpsc_send message: {}", e);
                    };
                    cmd_input = true;
                    buffer.clear();
                    continue;
                } else {
                    if let Some(mut key) = gen_key(&recorded_input, &input_details) {
                        buffer.append(&mut key);
                        if let None = kbt.get_node(&buffer) {
                            buffer.clear();
                        }
                        else {
                            // TODO: Find efficient way of detecting AppQuit, currently for this one detection
                            // all trait objects of UserEvent are made to have get_name()
                            if let Some(ue) = kbt.get(&buffer) {
                                exec_cmd(mpsc_send.clone(), Some(oneshot_tx), ue).await;

                                // If AppQuit, leave.
                                if ue.eq("app_quit") {
                                    break;
                                } 
                                buffer.clear();
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
} 

//TODO: Test function to see if all keys are covered and all possibilities handled.

// Generate KeyCombination for NcReceived & NcInput.
fn gen_key(ncr: &NcReceived, id: &NcInput) -> Option<KeyCombination> {
    if id.evtype == 3 { return None; } // Ignore Kitty release events.
    let mut key_comb_vec: Vec<Key> = Vec::new();
    if id.ctrl {key_comb_vec.push(Key::HoldCtrl); }
    if id.alt {key_comb_vec.push(Key::HoldAlt); }
    match ncr {
        NcReceived::Char(ch) => {
            for ch_lower in ch.to_lowercase() {
                if id.shift || (*ch != ch_lower && !id.ctrl) { key_comb_vec.push(Key::HoldShift); };
                match ch_lower {
                    'a' => {  key_comb_vec.push(Key::KeyA); }, 
                    'b' => {  key_comb_vec.push(Key::KeyB); }, 
                    'c' => {  key_comb_vec.push(Key::KeyC); }, 
                    'd' => {  key_comb_vec.push(Key::KeyD); }, 
                    'e' => {  key_comb_vec.push(Key::KeyE); }, 
                    'f' => {  key_comb_vec.push(Key::KeyF); }, 
                    'g' => {  key_comb_vec.push(Key::KeyG); }, 
                    'h' => {  key_comb_vec.push(Key::KeyH); }, 
                    'i' => {  key_comb_vec.push(Key::KeyI); }, 
                    'j' => {  key_comb_vec.push(Key::KeyJ); }, 
                    'k' => {  key_comb_vec.push(Key::KeyK); }, 
                    'l' => {  key_comb_vec.push(Key::KeyL); }, 
                    'm' => {  key_comb_vec.push(Key::KeyM); }, 
                    'n' => {  key_comb_vec.push(Key::KeyN); }, 
                    'o' => {  key_comb_vec.push(Key::KeyO); }, 
                    'p' => {  key_comb_vec.push(Key::KeyP); }, 
                    'q' => {  key_comb_vec.push(Key::KeyQ); }, 
                    'r' => {  key_comb_vec.push(Key::KeyR); }, 
                    's' => {  key_comb_vec.push(Key::KeyS); }, 
                    't' => {  key_comb_vec.push(Key::KeyT); }, 
                    'u' => {  key_comb_vec.push(Key::KeyU); }, 
                    'v' => {  key_comb_vec.push(Key::KeyV); }, 
                    'w' => {  key_comb_vec.push(Key::KeyW); }, 
                    'x' => {  key_comb_vec.push(Key::KeyX); }, 
                    'y' => {  key_comb_vec.push(Key::KeyY); }, 
                    'z' => {  key_comb_vec.push(Key::KeyZ); }, 
                    _ => { warn!{"Found no key matching char: {}", ch}; return None; }  
                }
            }
            return Some(key_comb_vec);
        },
        NcReceived::Event(key) => {
            match key { 
                &NcKey::Enter => { key_comb_vec.push(Key::KeyEnter); },
                &NcKey::Esc => { key_comb_vec.push(Key::KeyEsc); },
                &NcKey::Space => { key_comb_vec.push(Key::KeySpace); },
                &NcKey::Backspace => { key_comb_vec.push(Key::KeyBackspace); },
                &NcKey::Tab => { key_comb_vec.push(Key::KeyTab); },
                &NcKey::Up => { key_comb_vec.push(Key::KeyUp); },
                &NcKey::Down => { key_comb_vec.push(Key::KeyDown); },
                &NcKey::Left => { key_comb_vec.push(Key::KeyLeft); },
                &NcKey::Right => { key_comb_vec.push(Key::KeyRight); },
                &NcKey::F01 => { key_comb_vec.push(Key::KeyF1); },
                &NcKey::F02 => { key_comb_vec.push(Key::KeyF2); },
                &NcKey::F03 => { key_comb_vec.push(Key::KeyF3); },
                &NcKey::F04 => { key_comb_vec.push(Key::KeyF4); },
                &NcKey::F05 => { key_comb_vec.push(Key::KeyF5); },
                &NcKey::F06 => { key_comb_vec.push(Key::KeyF6); },
                &NcKey::F07 => { key_comb_vec.push(Key::KeyF7); },
                &NcKey::F08 => { key_comb_vec.push(Key::KeyF8); },
                &NcKey::F09 => { key_comb_vec.push(Key::KeyF9); },
                &NcKey::F10 => { key_comb_vec.push(Key::KeyF10); },
                &NcKey::F11 => { key_comb_vec.push(Key::KeyF11); },
                &NcKey::F12 => { key_comb_vec.push(Key::KeyF12); },
                &NcKey::Ins => { key_comb_vec.push(Key::KeyInsert); },
                &NcKey::Del => { key_comb_vec.push(Key::KeyDel); },
                &NcKey::Home => { key_comb_vec.push(Key::KeyHome); },
                &NcKey::End => { key_comb_vec.push(Key::KeyEnd); },
                &NcKey::PgUp => { key_comb_vec.push(Key::KeyPageUp); },
                &NcKey::PgDown => { key_comb_vec.push(Key::KeyPageDown); },
                _ => { 
                    debug!("User input: {:?} {:?}", ncr, id);
                    warn!("Found no key matching for event."); return None;
                }
            }
            return Some(key_comb_vec);
        },
        _ => { return None; }
    }
}
