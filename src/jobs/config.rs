use anyhow::{ anyhow, Result };
use log::{ error, info, warn };
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};
use std::collections::HashMap;


use crate::def::app::CONFIG_DIR_PATHS;
use crate::events::get_user_event;
use super::util::{
    config::{ Config, ThemeDes, TUIPrefsDes },
    key_bindings::{ DEFAULT_KEY_BINDINGS, Key, KeyBindingsTrie, KeyCombination, STRING_TO_KEYS }
};

pub async fn load_config() -> Config {
    for path in CONFIG_DIR_PATHS {
        match File::open(Path::new(&format!("{}{}", path, "/Config.toml"))) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(config) = deserialize_toml(&contents) {
                        info!("Using config at: {}", path);
                        return config
                    }
                }
                else { warn!("Unable to open file: {}", path); }
            }
            Err(e) => { warn!("Skipping opening file: {}, due to: {:?}", path, e); }
        }
    }
    error!("Returning default config.");
    Config::default()
}

pub async fn create_key_bindings_trie(kb: &HashMap<String, String>) -> Result<KeyBindingsTrie> {
    info!("Creating key-bindings trie.");
    let mut kb_trie: KeyBindingsTrie = KeyBindingsTrie::new();
    for (&key, &def_val) in DEFAULT_KEY_BINDINGS.entries() {
        if let Some(val) = kb.get(key) {
            kb_trie.insert_owned(parse_to_key_combination(val)?, get_user_event(key)?);
        }  
        else {
            kb_trie.insert_owned(parse_to_key_combination(def_val)?, get_user_event(key)?);
        }
    }
    Ok(kb_trie)
}

fn deserialize_toml(s: &str) -> Result<Config, toml::de::Error> {
    match toml::from_str(s) {
        Ok(toml) => { Ok(toml) }
        Err(e) => { 
            error!("Error deserializing: {:?}", e); 
            Err(e) 
        }
    }
}

fn parse_to_key_combination(key_comb_str: &str) -> Result<KeyCombination> {    
    let mut key_comb: Vec<Key> = Vec::new();
    let find_key = |s: &str| -> Result<Key> {
            if let Some(key) = STRING_TO_KEYS.get(s) { Ok(key.clone()) } 
            else { Err(anyhow!(format!("Cannot find any key corresponding to {}", s))) } 
    };
    let mut is_special_key: bool = false;
    let mut special_key_index = 0;
    for (i, c) in key_comb_str.chars().enumerate() {
        match c {
            '<' => { 
                if is_special_key { return Err(anyhow!("Invalid key-binding format {}, < wrongly placed", key_comb_str)); }
                is_special_key = true; special_key_index = i+1; 
            },
            '-' => {
                if !is_special_key { return Err(anyhow!("Invalid key-binding format {}, - wrongly placed", key_comb_str)); }
                key_comb.push(find_key(&key_comb_str[special_key_index..i])?);
                special_key_index = i+1;
            },
            '>' => {
                if !is_special_key { return Err(anyhow!("Invalid key-binding format {}, > wrongly placed", key_comb_str)); }
                key_comb.push(find_key(&key_comb_str[special_key_index..i])?);
                is_special_key = false; 
            },
            _ =>  { 
                if is_special_key { continue; }
                else { key_comb.push(find_key(&c.to_string())?); } 
            }
        }
    }
    if is_special_key { return Err(anyhow!("Invalid key-binding format {}", key_comb_str)); }
    Ok(key_comb)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ Config, Key, KeyCombination, deserialize_toml, parse_to_key_combination, ThemeDes, TUIPrefsDes  };
// Test deserialize_toml deserializes toml proper.
    #[test]
    fn test_deserialize_toml() {
        let res_config: Config = deserialize_toml(r#"
            [key-bindings]
            app_quit = "abcdefghi"

            [tui]
            theme.highlight-fg= "222222"
            theme.highlight-bg= "333333"
        "#).unwrap();
        // let mut exp_config = Config::default();
        // exp_config.key_bindings.app_quit = "ABCDEFGHIJ".to_owned();
        assert_eq!(res_config, Config {
            key_bindings: HashMap::from([
                ("app_quit".to_owned(), "abcdefghi".to_owned())
            ]),
            tui: TUIPrefsDes {
                theme: ThemeDes {
                    highlight_fg: "222222".to_string(),
                    highlight_bg: "333333".to_string()
                }
            }
        });
        // exp_config.key_bindings.app_quit = "ABC".to_owned();
        // assert_ne!(res_config, exp_config);
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
