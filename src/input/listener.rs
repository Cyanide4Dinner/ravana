use anyhow::Result;
use libnotcurses_sys::{
    c_api::notcurses_inputready_fd,
    Nc,
    NcInput,
    NcKey,
    NcReceived,
}; 
use log::{ error, warn };
use nix::poll::{ poll, PollFd, PollFlags };
use std::sync::{ Arc, Mutex };

use crate::{ 
    tools::log_err_desc,
    tui::{ App, AppRes, cmd_plt_val_input }
};
use super::{ 
    command_to_event::exec_cmd,
    util::key_bindings::{ 
        Key,
        KeyBindingsTrie,
        KeyCombination,
    }
};

// -----------------------------------------------------------------------------------------------------------
// * Listen for user input by polling.
// * Return event.
// -----------------------------------------------------------------------------------------------------------
pub fn listen(nc: Arc<Mutex<&mut Nc>>, kbt: KeyBindingsTrie, app: &mut App) -> Result<()> {
    let mut buffer: KeyCombination = KeyCombination::new();
    let mut input_details = NcInput::new_empty();

    // Command mode - Enter command in palette.
    let mut cmd_mode: bool = false;

    let mut nc_lock = nc.lock().unwrap();
    let input_fd = PollFd::new(
        unsafe { notcurses_inputready_fd(*nc_lock as &mut Nc as *mut Nc) },
        PollFlags::POLLIN);
    drop(nc_lock);

    loop {
        if let Ok(_) = poll(&mut [input_fd], -1) {
            nc_lock = nc.lock().unwrap();
            let recorded_input = nc_lock.get_nblock(Some(&mut input_details))?;
            drop(nc_lock);

            if !handle_input(
                    app,
                    &mut buffer,
                    &mut cmd_mode,
                    &recorded_input,
                    &mut input_details,
                    &kbt
                )? {
                break
            };

            // // -----------------------------------------------------------------------------------------------
            // // Cmd mode - true
            // // -----------------------------------------------------------------------------------------------
            // if cmd_mode {
            //     match recorded_input {
            //         // Execute command.
            //         NcReceived::Event(NcKey::Enter) => {
            //             cmd_mode = false;
            //             match app.exec_cmd() {
            //                 Ok(Some(AppRes::AppQuit)) => { break; },
            //                 Err(e) => { error!("Unable to execute command {}", e); },
            //                 _ => {  }
            //             };
            //             continue;
            //         },
            //
            //         // Escape command mode.
            //         NcReceived::Event(NcKey::Esc) => {
            //             cmd_mode = false;
            //             log_err_desc!(app.exit_cmd(), "Unable to exit command palette");
            //             continue;
            //         },
            //
            //         _ => {
            //             // Validate if input recieved is compatible.
            //             if cmd_plt_val_input(&recorded_input) {
            //                 match app.input_cmd_plt(input_details.clone()) {
            //                     Ok(AppRes::CmdModeCont) => {
            //                         continue;
            //                     },
            //                     Ok(AppRes::CmdModeQuit) => {
            //                         cmd_mode = false;
            //                         continue;
            //                     },
            //                     Err(e) => {
            //                         error!("{}", e);
            //                     },
            //                     Ok(ar) => {
            //                         error!("Invalid return from App to listener {:?}", ar)
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }
            // // -----------------------------------------------------------------------------------------------
            // // Cmd mode - false
            // // -----------------------------------------------------------------------------------------------
            // else {
            //     if let NcReceived::Char(':') = recorded_input {
            //         cmd_mode = true;
            //         log_err_desc!(app.enter_cmd(), "Unable to enter command palette");
            //         buffer.clear();
            //         continue;
            //     } else {
            //         if let Some(mut key) = gen_key(&recorded_input, &input_details) {
            //             buffer.append(&mut key);
            //             if let None = kbt.get_node(&buffer) {
            //                 buffer.clear();
            //             }
            //             else {
            //                 // TODO: Find efficient way of detecting AppQuit, currently for this one detection
            //                 // all trait objects of UserEvent are made to have get_name()
            //                 if let Some(cmd) = kbt.get(&buffer) {
            //                     log_err_desc!(exec_cmd(app, cmd), "");
            //
            //                     // If AppQuit, leave.
            //                     if cmd.eq("app_quit") {
            //                         break;
            //                     }
            //                     buffer.clear();
            //                 }
            //             }
            //         }
            //     }
            // }
        }
    }

    Ok(())
}

// Separately handle input for better testability.
pub fn handle_input(
    app: &mut App,
    buffer: &mut KeyCombination,
    cmd_mode: &mut bool,
    recorded_input: &NcReceived,
    input_details: &mut NcInput,
    kbt: &KeyBindingsTrie) -> Result<bool> { // true for continue, false for break
    
    // -----------------------------------------------------------------------------------------------
    // Cmd mode - true
    // -----------------------------------------------------------------------------------------------
    if *cmd_mode {
        match recorded_input {
            // Execute command.
            NcReceived::Event(NcKey::Enter) => {
                *cmd_mode = false;
                match app.exec_cmd() {
                    Ok(Some(AppRes::AppQuit)) => { return Ok(false); },
                    Err(e) => { error!("Unable to execute command {}", e); },
                    _ => {  }
                };
                return Ok(true);
            },

            // Escape command mode.
            NcReceived::Event(NcKey::Esc) => {
                *cmd_mode = false;
                log_err_desc!(app.exit_cmd(), "Unable to exit command palette");
                return Ok(true);
            },

            _ => {
                // Validate if input recieved is compatible.
                if cmd_plt_val_input(&recorded_input) { 
                    match app.input_cmd_plt(input_details.clone()) {
                        Ok(AppRes::CmdModeCont) => {
                            return Ok(true);
                        },
                        Ok(AppRes::CmdModeQuit) => {
                            *cmd_mode = false;
                            return Ok(true);
                        },
                        Err(e) => {
                            error!("{}", e);
                        },
                        Ok(ar) => {
                            error!("Invalid return from App to listener {:?}", ar)
                        }
                    }
                }
            }
        }
    } 
    // -----------------------------------------------------------------------------------------------
    // Cmd mode - false
    // -----------------------------------------------------------------------------------------------
    else {
        if let NcReceived::Char(':') = recorded_input {
            *cmd_mode = true;
            log_err_desc!(app.enter_cmd(), "Unable to enter command palette");
            buffer.clear();
            return Ok(true);
        } else {
            if let Some(mut key) = gen_key(&recorded_input, &input_details) {
                buffer.append(&mut key);
                if let None = kbt.get_node(buffer as &KeyCombination) {
                    buffer.clear();
                }
                else {
                    // TODO: Find efficient way of detecting AppQuit, currently for this one detection
                    // all trait objects of UserEvent are made to have get_name()
                    if let Some(cmd) = kbt.get(buffer as &KeyCombination) {
                        log_err_desc!(exec_cmd(app, cmd), "");

                        // If AppQuit, leave.
                        if cmd.eq("app_quit") {
                            return Ok(false);
                        } 
                        buffer.clear();
                    }
                }
            }
        }
    }
    Ok(true)
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
                    warn!("Found no key matching for event."); return None;
                }
            }
            return Some(key_comb_vec);
        },
        _ => { return None; }
    }
}
