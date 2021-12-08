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
    use radix_trie::{ Trie, TrieKey };
    #[cfg(test)]
    use enum_iterator::IntoEnumIterator;

    use crate::events::UserEvent;
    
    #[cfg_attr(test, derive(IntoEnumIterator))]
    #[derive(Debug, Eq, PartialEq)]
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
        KeyCtrl,
        KeyShift,
        KeyAlt,
        HoldCtrl,
        HoldShift,
        HoldAlt,
    }

    pub(super) fn code_to_key(code: &u8) -> Result<Key> {
        match code {
            0x01u8 => Ok(Key::KeyA), 
            0x02u8 => Ok(Key::KeyB),
            0x03u8 => Ok(Key::KeyC),
            0x04u8 => Ok(Key::KeyD),
            0x05u8 => Ok(Key::KeyE),
            0x06u8 => Ok(Key::KeyF),
            0x07u8 => Ok(Key::KeyG),
            0x08u8 => Ok(Key::KeyH),
            0x09u8 => Ok(Key::KeyI),
            0x0au8 => Ok(Key::KeyJ),
            0x0bu8 => Ok(Key::KeyJ),
            0x0cu8 => Ok(Key::KeyK),
            0x0du8 => Ok(Key::KeyL),
            0x0eu8 => Ok(Key::KeyM),
            0x0fu8 => Ok(Key::KeyN),
            0x10u8 => Ok(Key::KeyO),
            0x10u8 => Ok(Key::KeyP),
            0x11u8 => Ok(Key::KeyQ),
            0x12u8 => Ok(Key::KeyR),
            0x13u8 => Ok(Key::KeyS),
            0x14u8 => Ok(Key::KeyT),
            0x15u8 => Ok(Key::KeyU),
            0x16u8 => Ok(Key::KeyV),
            0x17u8 => Ok(Key::KeyW),
            0x18u8 => Ok(Key::KeyX),
            0x19u8 => Ok(Key::KeyY),
            0x1au8 => Ok(Key::KeyZ),
            0x1bu8 => Ok(Key::KeyCtrl),
            0x1cu8 => Ok(Key::KeyShift),
            0x1du8 => Ok(Key::KeyAlt),
            0x1eu8 => Ok(Key::HoldCtrl),
            0x1fu8 => Ok(Key::HoldShift),
            0x20u8 => Ok(Key::HoldAlt),
            _ => Err(anyhow!("Invalid u8 key code: {}", code))
        }
        
    }

    pub(super) fn key_to_code(key: &Key) -> Result<u8> {
        match key {
            KeyA => Ok(0x01u8), 
            KeyB => Ok(0x02u8),
            KeyC => Ok(0x03u8),
            KeyD => Ok(0x04u8),
            KeyE => Ok(0x05u8),
            KeyF => Ok(0x06u8),
            KeyG => Ok(0x07u8),
            KeyH => Ok(0x08u8),
            KeyI => Ok(0x09u8),
            KeyJ => Ok(0x0au8),
            KeyK => Ok(0x0bu8),
            KeyL => Ok(0x0cu8),
            KeyM => Ok(0x0du8),
            KeyN => Ok(0x0eu8),
            KeyO => Ok(0x0fu8),
            KeyP => Ok(0x10u8),
            KeyQ => Ok(0x11u8),
            KeyR => Ok(0x12u8),
            KeyS => Ok(0x13u8),
            KeyT => Ok(0x14u8),
            KeyU => Ok(0x15u8),
            KeyV => Ok(0x16u8),
            KeyW => Ok(0x17u8),
            KeyX => Ok(0x18u8),
            KeyY => Ok(0x19u8),
            KeyZ => Ok(0x1au8),
            KeyCtrl => Ok(0x1bu8),
            KeyShift => Ok(0x1cu8),
            KeyAlt => Ok(0x1du8),
            HoldCtrl => Ok(0x1eu8),
            HoldShift => Ok(0x1fu8),
            HoldAlt => Ok(0x20u8),
            _ => Err(anyhow!("Invalid Key enum: {:?}", key))
        }
    }

    type KeyCombination = Vec<Key>;

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
    pub struct KeyBindingsTrie {
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
    use super::key_bindings::{ Key, key_to_code, code_to_key };

    #[test]
    fn test_default_config() -> Result<()> {
        let mut file = File::open(Path::new("docs/.ravana/Config.toml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = toml::from_str(&*contents)?;  
        assert_eq!(config, Config::default());
        Ok(()) 
    }

    #[test]
    fn test_code_key_enum_conversions() {
        let itr = Key::into_iter();
        for _ in [0..Key::VARIANT_COUNT] {
            if let Some(key) = itr.next() {  
                assert_eq!(code_to_key(key_to_code(key).unwrap()).unwrap(), key);
            }
            else { panic!("No key returned by iterator") }
        } 
    }
}
