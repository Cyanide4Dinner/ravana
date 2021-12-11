pub mod config {
    use serde::Deserialize;

    #[derive(Deserialize, Debug, PartialEq, Eq, Default)]
    #[serde(rename_all(deserialize = "kebab-case"),default)]
    pub struct Config {
        pub key_bindings: KeyBindings
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(default)]
    pub struct KeyBindings {
        pub app_quit: String,
    }

    impl Default for KeyBindings {
        fn default() -> KeyBindings {
            KeyBindings {
                app_quit: "ZZ".to_string()
            }
        }
    }
}

pub mod key_bindings {
    use anyhow::{ anyhow, Result, Context };
    use phf::{ phf_map, Map };
    use radix_trie::{ Trie, TrieKey };
    use std::collections::HashMap;
    #[cfg(test)]
    use enum_iterator::IntoEnumIterator;

    use crate::events::UserEvent;
    
    #[cfg_attr(test, derive(IntoEnumIterator))]
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum Key {
        KeyA,
        KeyB,
        KeyC,
        KeyD,
        KeyE,
        KeyF,
        KeyG,
        KeyH,
        KeyI,
        KeyJ,
        KeyK,
        KeyL,
        KeyM,
        KeyN,
        KeyO,
        KeyP,
        KeyQ,
        KeyR,
        KeyS,
        KeyT,
        KeyU,
        KeyV,
        KeyW,
        KeyX,
        KeyY,
        KeyZ,
        KeyEnter,
        KeyEsc,
        KeySpace,
        KeyBackspace,
        KeyTab,
        KeyUp,
        KeyDown,
        KeyLeft,
        KeyRight,
        KeyF1,
        KeyF2,
        KeyF3,
        KeyF4,
        KeyF5,
        KeyF6,
        KeyF7,
        KeyF8,
        KeyF9,
        KeyF10,
        KeyF11,
        KeyF12,
        KeyInsert,
        KeyDel,
        KeyHome,
        KeyEnd,
        KeyPageUp,
        KeyPageDown,
        HoldCtrl,
        HoldShift,
        HoldAlt,
    }

    pub const STRING_TO_KEYS: Map<&'static str, Key> = phf_map!{
        "a" => Key::KeyA,
        "b" => Key::KeyB,
        "c" => Key::KeyC,
        "d" => Key::KeyD,
        "e" => Key::KeyE,
        "f" => Key::KeyF,
        "g" => Key::KeyG,
        "h" => Key::KeyH,
        "i" => Key::KeyI,
        "j" => Key::KeyJ,
        "l" => Key::KeyL,
        "m" => Key::KeyM,
        "n" => Key::KeyN,
        "o" => Key::KeyO,
        "p" => Key::KeyP,
        "q" => Key::KeyQ,
        "r" => Key::KeyR,
        "s" => Key::KeyS,
        "t" => Key::KeyT,
        "u" => Key::KeyU,
        "v" => Key::KeyV,
        "w" => Key::KeyW,
        "x" => Key::KeyX,
        "y" => Key::KeyY,
        "z" => Key::KeyZ,
        "CR" => Key::KeyEnter,
        "Enter" => Key::KeyEnter,
        "Return" => Key::KeyEnter,
        "Esc" => Key::KeyEsc,
        "Space" => Key::KeySpace,
        "Tab" => Key::KeyTab,
        "BS" => Key::KeyBackspace,
        "Up" => Key::KeyUp,
        "Down" => Key::KeyDown,
        "Left" => Key::KeyLeft,
        "Right" => Key::KeyRight,
        "F1" => Key::KeyF1,
        "F2" => Key::KeyF2,
        "F3" => Key::KeyF3,
        "F4" => Key::KeyF4,
        "F5" => Key::KeyF5,
        "F6" => Key::KeyF6,
        "F7" => Key::KeyF7,
        "F8" => Key::KeyF8,
        "F9" => Key::KeyF9,
        "F10" => Key::KeyF10,
        "F11" => Key::KeyF11,
        "F12" => Key::KeyF12,
        "Insert" => Key::KeyInsert,
        "Del" => Key::KeyDel,
        "Home" => Key::KeyHome,
        "End" => Key::KeyEnd,
        "PageUp" => Key::KeyPageUp,
        "PageDown" => Key::KeyPageDown,
        "C" => Key::HoldCtrl,
        "S" => Key::HoldShift,
        "A" => Key::HoldAlt,
        "M" => Key::HoldAlt,
    };

    // pub(super) fn code_to_key(code: &u8) -> Result<Key> {
    //     match code {
    //         0x01u8 => Ok(Key::KeyA),
    //         0x02u8 => Ok(Key::KeyB),
    //         0x03u8 => Ok(Key::KeyC),
    //         0x04u8 => Ok(Key::KeyD),
    //         0x05u8 => Ok(Key::KeyE),
    //         0x06u8 => Ok(Key::KeyF),
    //         0x07u8 => Ok(Key::KeyG),
    //         0x08u8 => Ok(Key::KeyH),
    //         0x09u8 => Ok(Key::KeyI),
    //         0x0au8 => Ok(Key::KeyJ),
    //         0x0bu8 => Ok(Key::KeyJ),
    //         0x0cu8 => Ok(Key::KeyK),
    //         0x0du8 => Ok(Key::KeyL),
    //         0x0eu8 => Ok(Key::KeyM),
    //         0x0fu8 => Ok(Key::KeyN),
    //         0x10u8 => Ok(Key::KeyO),
    //         0x10u8 => Ok(Key::KeyP),
    //         0x11u8 => Ok(Key::KeyQ),
    //         0x12u8 => Ok(Key::KeyR),
    //         0x13u8 => Ok(Key::KeyS),
    //         0x14u8 => Ok(Key::KeyT),
    //         0x15u8 => Ok(Key::KeyU),
    //         0x16u8 => Ok(Key::KeyV),
    //         0x17u8 => Ok(Key::KeyW),
    //         0x18u8 => Ok(Key::KeyX),
    //         0x19u8 => Ok(Key::KeyY),
    //         0x1au8 => Ok(Key::KeyZ),
    //         0x1bu8 => Ok(Key::KeyEnter),
    //         0x1cu8 => Ok(Key::KeyEsc),
    //         0x1du8 => Ok(Key::KeySpace),
    //         0x1eu8 => Ok(Key::KeyBackspace),
    //         0x1fu8 => Ok(Key::KeyTab),
    //         0x20u8 => Ok(Key::KeyUp),
    //         0x21u8 => Ok(Key::KeyDown),
    //         0x22u8 => Ok(Key::KeyLeft),
    //         0x23u8 => Ok(Key::KeyRight),
    //         0x24u8 => Ok(Key::KeyF1),
    //         0x25u8 => Ok(Key::KeyF2),
    //         0x26u8 => Ok(Key::KeyF3),
    //         0x27u8 => Ok(Key::KeyF4),
    //         0x28u8 => Ok(Key::KeyF5),
    //         0x29u8 => Ok(Key::KeyF6),
    //         0x2au8 => Ok(Key::KeyF7),
    //         0x2bu8 => Ok(Key::KeyF8),
    //         0x2cu8 => Ok(Key::KeyF9),
    //         0x2du8 => Ok(Key::KeyF10),
    //         0x2eu8 => Ok(Key::KeyF11),
    //         0x2fu8 => Ok(Key::KeyF12),
    //         0x30u8 => Ok(Key::KeyInsert),
    //         0x31u8 => Ok(Key::KeyDel),
    //         0x32u8 => Ok(Key::KeyHome),
    //         0x33u8 => Ok(Key::KeyEnd),
    //         0x34u8 => Ok(Key::KeyPageUp),
    //         0x35u8 => Ok(Key::KeyPageDown),
    //         0x36u8 => Ok(Key::HoldCtrl),
    //         0x37u8 => Ok(Key::HoldShift),
    //         0x38u8 => Ok(Key::HoldAlt),
    //         _ => Err(anyhow!("Invalid u8 key code: {}", code))
    //     }
    // }
    //
    // pub(super) fn key_to_code(key: &Key) -> Result<u8> {
    //     match key {
    //         KeyA => Ok(0x01u8),
    //         KeyB => Ok(0x02u8),
    //         KeyC => Ok(0x03u8),
    //         KeyD => Ok(0x04u8),
    //         KeyE => Ok(0x05u8),
    //         KeyF => Ok(0x06u8),
    //         KeyG => Ok(0x07u8),
    //         KeyH => Ok(0x08u8),
    //         KeyI => Ok(0x09u8),
    //         KeyJ => Ok(0x0au8),
    //         KeyK => Ok(0x0bu8),
    //         KeyL => Ok(0x0cu8),
    //         KeyM => Ok(0x0du8),
    //         KeyN => Ok(0x0eu8),
    //         KeyO => Ok(0x0fu8),
    //         KeyP => Ok(0x10u8),
    //         KeyQ => Ok(0x11u8),
    //         KeyR => Ok(0x12u8),
    //         KeyS => Ok(0x13u8),
    //         KeyT => Ok(0x14u8),
    //         KeyU => Ok(0x15u8),
    //         KeyV => Ok(0x16u8),
    //         KeyW => Ok(0x17u8),
    //         KeyX => Ok(0x18u8),
    //         KeyY => Ok(0x19u8),
    //         KeyZ => Ok(0x1au8),
    //         KeyEnter => Ok(0x1bu8),
    //         KeyEsc => Ok(0x1cu8),
    //         KeySpace => Ok(0x1du8),
    //         KeyBackspace => Ok(0x1eu8),
    //         KeyTab => Ok(0x1fu8),
    //         KeyUp => Ok(0x20u8),
    //         KeyDown => Ok(0x21u8),
    //         KeyLeft => Ok(0x22u8),
    //         KeyRight => Ok(0x23u8),
    //         KeyF1 => Ok(0x24u8),
    //         KeyF2 => Ok(0x25u8),
    //         KeyF3 => Ok(0x26u8),
    //         KeyF4 => Ok(0x27u8),
    //         KeyF5 => Ok(0x28u8),
    //         KeyF6 => Ok(0x29u8),
    //         KeyF7 => Ok(0x2au8),
    //         KeyF8 => Ok(0x2bu8),
    //         KeyF9 => Ok(0x2cu8),
    //         KeyF10 => Ok(0x2du8),
    //         KeyF11 => Ok(0x2eu8),
    //         KeyF12 => Ok(0x2fu8),
    //         KeyInsert => Ok(0x30u8),
    //         KeyDel => Ok(0x31u8),
    //         KeyHome => Ok(0x32u8),
    //         KeyEnd => Ok(0x33u8),
    //         KeyPageUp => Ok(0x34u8),
    //         KeyPageDown => Ok(0x35u8),
    //         HoldCtrl => Ok(0x36u8),
    //         HoldShift => Ok(0x37u8),
    //         HoldAlt => Ok(0x38u8),
    //         _ => Err(anyhow!("Invalid Key enum: {:?}", key))
    //     }
    // }

    pub type KeyCombination = Vec<Key>;

    // #[derive(Eq)]
    // pub struct KeyCombination(Vec<Key>);
    // impl PartialEq for KeyCombination {
    //     fn eq(&self, other: &Self) -> bool {
    //         assert_eq!(self.0.iter().eq(other.0.iter()), true);
    //         true
    //     }
    // }
    // impl TrieKey for KeyCombination {
    //     fn encode_bytes(&self) -> Vec<u8> {
    //         let mut key_code_vec: Vec<u8> = Vec::new();
    //         for key in &self.0 {
    //             key_code_vec.push(key_to_code(key).context(format!("key_to_code unable to find code for {:?}", key)).unwrap())
    //         }
    //         key_code_vec
    //     }
    // }
    //
    pub(super) struct KeyBindingsTrie {
        leader: Key,
        trie: Trie<KeyCombination, Box<dyn UserEvent>>
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use std::{
        fs::File,
        io::prelude::*,
        path::Path
    };

    use super::config::Config;

    #[test]
    fn test_default_config() -> Result<()> {
        let mut file = File::open(Path::new("docs/.ravana/Config.toml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&*contents)?;  
        assert_eq!(config, Config::default());
        Ok(()) 
    }

    // #[test]
    // fn test_code_key_enum_conversions() {
    //     let itr = Key::into_iter();
    //     for _ in [0..Key::VARIANT_COUNT] {
    //         if let Some(key) = itr.next() {
    //             assert_eq!(code_to_key(key_to_code(key).unwrap()).unwrap(), key);
    //         }
    //         else { panic!("No key returned by iterator") }
    //     }
    // }
}
