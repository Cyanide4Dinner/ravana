pub mod key_bindings {
    use anyhow::{ bail, Result };
    #[cfg(test)]
    use enum_iterator::IntoEnumIterator; // Required in a unit test.
    use phf::{ phf_map, Map };
    use sequence_trie::SequenceTrie;
    use std::collections::HashMap;

    // Supported keys.
    #[cfg_attr(test, derive(IntoEnumIterator))]
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

    // Stirng literals in key-combination string format in Config.toml
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


    #[cfg(test)]
    use anyhow::anyhow;

    // For a unit test.
    #[cfg(test)]
    #[allow(unreachable_patterns)]
    pub(super) fn key_to_code(key: &Key) -> Result<u8> {
        match key {
            Key::KeyA => Ok(0x01u8),
            Key::KeyB => Ok(0x02u8),
            Key::KeyC => Ok(0x03u8),
            Key::KeyD => Ok(0x04u8),
            Key::KeyE => Ok(0x05u8),
            Key::KeyF => Ok(0x06u8),
            Key::KeyG => Ok(0x07u8),
            Key::KeyH => Ok(0x08u8),
            Key::KeyI => Ok(0x09u8),
            Key::KeyJ => Ok(0x0au8),
            Key::KeyK => Ok(0x0bu8),
            Key::KeyL => Ok(0x0cu8),
            Key::KeyM => Ok(0x0du8),
            Key::KeyN => Ok(0x0eu8),
            Key::KeyO => Ok(0x0fu8),
            Key::KeyP => Ok(0x10u8),
            Key::KeyQ => Ok(0x11u8),
            Key::KeyR => Ok(0x12u8),
            Key::KeyS => Ok(0x13u8),
            Key::KeyT => Ok(0x14u8),
            Key::KeyU => Ok(0x15u8),
            Key::KeyV => Ok(0x16u8),
            Key::KeyW => Ok(0x17u8),
            Key::KeyX => Ok(0x18u8),
            Key::KeyY => Ok(0x19u8),
            Key::KeyZ => Ok(0x1au8),
            Key::KeyEnter => Ok(0x1bu8),
            Key::KeyEsc => Ok(0x1cu8),
            Key::KeySpace => Ok(0x1du8),
            Key::KeyBackspace => Ok(0x1eu8),
            Key::KeyTab => Ok(0x1fu8),
            Key::KeyUp => Ok(0x20u8),
            Key::KeyDown => Ok(0x21u8),
            Key::KeyLeft => Ok(0x22u8),
            Key::KeyRight => Ok(0x23u8),
            Key::KeyF1 => Ok(0x24u8),
            Key::KeyF2 => Ok(0x25u8),
            Key::KeyF3 => Ok(0x26u8),
            Key::KeyF4 => Ok(0x27u8),
            Key::KeyF5 => Ok(0x28u8),
            Key::KeyF6 => Ok(0x29u8),
            Key::KeyF7 => Ok(0x2au8),
            Key::KeyF8 => Ok(0x2bu8),
            Key::KeyF9 => Ok(0x2cu8),
            Key::KeyF10 => Ok(0x2du8),
            Key::KeyF11 => Ok(0x2eu8),
            Key::KeyF12 => Ok(0x2fu8),
            Key::KeyInsert => Ok(0x30u8),
            Key::KeyDel => Ok(0x31u8),
            Key::KeyHome => Ok(0x32u8),
            Key::KeyEnd => Ok(0x33u8),
            Key::KeyPageUp => Ok(0x34u8),
            Key::KeyPageDown => Ok(0x35u8),
            Key::HoldCtrl => Ok(0x36u8),
            Key::HoldShift => Ok(0x37u8),
            Key::HoldAlt => Ok(0x38u8),
            _ => Err(anyhow!("Invalid Key enum: {:?}", key))
        }
    }

    // Default map for key-bindings ( field-name -> key-binding )
    pub const DEFAULT_KEY_BINDINGS: Map<&'static str, &'static str> = phf_map!{
        "app_quit" => "zz",
    };

    pub type KeyCombination = Vec<Key>;

    pub type KeyBindingsTrie = SequenceTrie<Key, String>;

    pub fn create_key_bindings_trie(kb: &HashMap<String, String>) -> Result<KeyBindingsTrie> {
        let mut kb_trie: KeyBindingsTrie = KeyBindingsTrie::new();
        for (&key, &def_val) in DEFAULT_KEY_BINDINGS.entries() {
            if let Some(val) = kb.get(key) {
                kb_trie.insert_owned(parse_to_key_combination(val)?, key.to_string());
            }  
            else {
                kb_trie.insert_owned(parse_to_key_combination(def_val)?, key.to_string());
            }
        }
        Ok(kb_trie)
    }

    // Parse key combination string to KeyCombination.
    pub(super) fn parse_to_key_combination(key_comb_str: &str) -> Result<KeyCombination> {    
        let mut key_comb: Vec<Key> = Vec::new();
        let find_key = |s: &str| -> Result<Key> {
                if let Some(key) = STRING_TO_KEYS.get(s) { Ok(key.clone()) } 
                else { 
                    bail!(format!("Cannot find any key corresponding to {}", s))
                } 
        };
        let mut is_special_key: bool = false;
        let mut special_key_index = 0;
        for (i, c) in key_comb_str.chars().enumerate() {
            match c {
                '<' => { 
                    if is_special_key { 
                       bail!("Invalid key-binding format {}, < wrongly placed", key_comb_str)
                    }
                    is_special_key = true; special_key_index = i+1; 
                },
                '-' => {
                    if !is_special_key { 
                        bail!("Invalid key-binding format {}, < wrongly placed", key_comb_str)
                    }
                    key_comb.push(find_key(&key_comb_str[special_key_index..i])?);
                    special_key_index = i+1;
                },
                '>' => {
                    if !is_special_key { 
                        bail!("Invalid key-binding format {}, < wrongly placed", key_comb_str)
                    }
                    key_comb.push(find_key(&key_comb_str[special_key_index..i])?);
                    is_special_key = false; 
                },
                _ =>  { 
                    if is_special_key { continue; }
                    else { key_comb.push(find_key(&c.to_string())?); } 
                }
            }
        }
        if is_special_key { 
            bail!("Invalid key-binding format {}, < wrongly placed", key_comb_str)
        }
        Ok(key_comb)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::{ anyhow, Result };
    use enum_iterator::IntoEnumIterator;
    use std::{
        collections::{ HashMap },
        fs::File,
        io::prelude::*,
        path::Path
    };

    use crate::jobs::Config;
    use super::key_bindings::{ DEFAULT_KEY_BINDINGS, Key, key_to_code };
    use super::key_bindings::{ KeyCombination, parse_to_key_combination };

    // * Test if all key-values pairs in DEFAULT_KEY_BINDINGS map and Config.toml match exactly
    #[test]
    fn test_default_key_bindings() -> Result<()> {
        let mut file = File::open(Path::new("docs/.ravana/Config.toml"))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let key_bindings_res: HashMap<String,String> = toml::from_str::<Config>(&*contents)?.key_bindings;  

        for res_key in key_bindings_res.keys() {
            if let Some(res_val) = key_bindings_res.get(res_key) {
                if let Some(exp_val) = DEFAULT_KEY_BINDINGS.get(&res_key) { assert_eq!(exp_val, res_val); }
                else { 
                    return Err(anyhow!("Cannot find key-value in default key-binding map for {}", res_key));
                }
            }
        }

        for exp_key in DEFAULT_KEY_BINDINGS.keys() {
            if let Some(exp_val) = DEFAULT_KEY_BINDINGS.get(exp_key) {
                if let Some(res_val) = key_bindings_res.get(exp_key.to_owned()) { assert_eq!(exp_val, res_val); } 
                else { 
                    return Err(anyhow!("Cannot find key-value in default Config.toml for {}", exp_key));
                }
            }
        }
        Ok(())
    }

    // * Test if all keys are present in key_to_code
    // * Test all codes must be different
    #[test]
    fn test_key_to_code() -> Result<()> {
        let mut key_codes: HashMap<u8, Key> = HashMap::new();
        for key in Key::into_enum_iter() {
            let code = key_to_code(&key)?;                            
            if key_codes.contains_key(&code) { 
                if let Some(matching_key) = key_codes.get(&code) { return Err(anyhow!("Matching code {} exists for another key: {:?} & {:?}", code, key, matching_key)); }
            }
            else { key_codes.insert(code, key); }
        }  
        Ok(())  
    }

    // Test parse_to_key_combination parses key combination strings to Key enum variants proper.
    #[test]
    fn test_parse_to_key_combination() {
        let key_comb1: KeyCombination = vec!{
            Key::HoldCtrl,
            Key::KeyB
        };
        assert_eq!(key_comb1, parse_to_key_combination("<C-b>").unwrap());

        let key_comb2: KeyCombination = vec!{
            Key::KeyEsc,
        }; 
        assert_eq!(key_comb2, parse_to_key_combination("<Esc>").unwrap());

        let key_comb3: KeyCombination = vec!{
            Key::HoldCtrl,
            Key::KeyTab
        };
        assert_eq!(key_comb3, parse_to_key_combination("<C-Tab>").unwrap());

        let key_comb4: KeyCombination = vec!{
            Key::HoldCtrl,
            Key::KeyA,
            Key::KeyG
        };
        assert_eq!(key_comb4, parse_to_key_combination("<C-a>g").unwrap());

        let key_comb5: KeyCombination = vec!{
            Key::KeyG,
            Key::KeyG
        };
        assert_eq!(key_comb5, parse_to_key_combination("gg").unwrap());

        let key_comb6: KeyCombination = vec!{
            Key::KeySpace,
            Key::KeyX
        };
        assert_eq!(key_comb6, parse_to_key_combination("<Space>x").unwrap());

        let key_comb7: KeyCombination = vec!{
            Key::KeyG,
            Key::KeyEsc
        };
        assert_eq!(key_comb7, parse_to_key_combination("g<Esc>").unwrap());
    }
}
