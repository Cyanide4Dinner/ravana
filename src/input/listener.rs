use anyhow::{ Context, Result };
use libnotcurses_sys::{
    Nc,
    NcInput,
    NcKey,
    NcReceived,
    NcTime
}; 
use log::{ info, warn };
use std::{ collections::HashMap, sync::{ Arc, Mutex } };
use tokio::sync::mpsc::Sender;

use crate::events::app_events::init_tui;
use crate::state::Message;
use super::util::key_bindings::{ 
    create_key_bindings_trie,
    Key,
    KeyBindingsTrie,
    KeyCombination,
};

pub async fn init(nc: Arc<Mutex<&mut Nc>>, kb: HashMap<String, String>, mpsc_send: Sender<Message>) -> Result<()> {
    info!("Init input listener.");
    let kbt = create_key_bindings_trie(&kb).context("Error parsing key-bindings.")?;
    init_tui(mpsc_send.clone()).await?; 
    listen(nc, kbt, mpsc_send.clone()).await?;
    Ok(())
}

//TODO: Create tests for event loop checking and ensuring.
async fn listen(nc: Arc<Mutex<&mut Nc>>, kbt: KeyBindingsTrie, mpsc_send: Sender<Message>) -> Result<()> {
    info!("Begin input listening loop.");
    let mut buffer: KeyCombination = KeyCombination::new();
    let mut input_details = NcInput::new_empty();
    loop {
        let mut nc_lock = nc.lock().unwrap(); // Lock Nc instance.
        if let Ok(()) = nc_lock.inputready_fd() {
            let recorded_input = nc_lock.get_nblock(Some(&mut input_details))?;
            drop(nc_lock); // Release the lock.
            if let Some(mut key) = gen_key(&recorded_input, &input_details) {
                buffer.append(&mut key); 
                if let None = kbt.get_node(&buffer) { 
                    buffer.clear();
                }
                else {
                    // TODO: Find efficient way of detecting AppQuit, currently for this one detection
                    // all trait objects of UserEvent are made to have get_name()
                    if let Some(ue) = kbt.get(&buffer) {
                        ue.trigger(mpsc_send.clone()).await?;
                        if ue.get_name().eq("AppQuit") {
                            return Ok(());
                        }
                        buffer.clear();
                    }
                }
            }
        }
    }
} 

//TODO: Test function to see if all keys are covered and all possibilities handled.
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
                    info!("User input: {:?} {:?}", ncr, id);
                    warn!("Found no key matching for event."); return None;
                }
            }
            return Some(key_comb_vec);
        },
        _ => { return None; }
    }
}
