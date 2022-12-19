use anyhow::{ anyhow, Result };
use log::{ error, info, warn };
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};

use crate::def::app::CONFIG_DIR_PATHS;
use super::util::{ config::Config, session::Session };

// -----------------------------------------------------------------------------------------------------------
// * Open up Config.toml for reading.
// * Deserialize to Config struct.
// -----------------------------------------------------------------------------------------------------------
pub fn load_config() -> Config {
    for path in CONFIG_DIR_PATHS {
        match File::open(Path::new(&format!("{}{}", path, "/Config.toml"))) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(config) = deserialize_config_toml(&contents) {
                        info!("Using config at: {}", path);
                        return config
                    }
                }
                else { warn!("Unable to open file: {}", path); }
            }
            Err(e) => { warn!("Skipping opening file: {}, due to: {:?}", path, e); }
        }
    }
    error!("Unable to open config file. Returning default config.");
    Config::default()
}

fn deserialize_config_toml(s: &str) -> Result<Config, toml::de::Error> {
    match toml::from_str(s) {
        Ok(toml) => { Ok(toml) }
        Err(e) => { 
            error!("Error deserializing: {:?}", e); 
            Err(e) 
        }
    }
}

pub fn get_session_file_path() -> Result<String> {
    for path in CONFIG_DIR_PATHS {
        if let Ok(file) = File::open(Path::new(&format!("{}{}", path, "/Session.toml"))) {
            info!("Using Session.toml at config: {}", path);
            return Ok(path.to_string());
        }
    }
    Err(anyhow!("Cannot find Session.toml."))
}

// -----------------------------------------------------------------------------------------------------------
// * Open and read Session.toml.
// * Convert to Session.
// -----------------------------------------------------------------------------------------------------------
pub fn load_session(session_file: &mut File) -> Session {
    let mut contents = String::new();
    if session_file.read_to_string(&mut contents).is_ok() {
        if let Ok(session) = deserialize_session_toml(&contents) {
            return session
        }
    }
    error!("Unable to open config file. Returning default config.");
    Session::default()
}

// -----------------------------------------------------------------------------------------------------------
// * Write Session object to Session.toml.
// -----------------------------------------------------------------------------------------------------------
pub fn save_session(session: &Session, session_file: &mut File) -> Result<()> {
    let session_text = toml::to_string(session)?;
    session_file.write_all(session_text.as_bytes())?;
    Ok(())
}

fn deserialize_session_toml(s: &str) -> Result<Session, toml::de::Error> {
    match toml::from_str(s) {
        Ok(toml) => { Ok(toml) }
        Err(e) => { 
            error!("Error deserializing: {:?}", e); 
            Err(e) 
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ Config, deserialize_config_toml };
    use crate::jobs::{ InterfaceDes, ThemeDes, TuiPrefsDes };

    // Test if deserialize_config_toml deserializes toml proper.
    #[test]
    fn test_deserialize_config_toml() {
        let res_config: Config = deserialize_config_toml(r##"
            [key-bindings]
            app_quit = "abcdefghi"

            [tui]
            interface.mouse-events-enable = false

            theme.highlight-fg = "#222222"
            theme.highlight-bg = "#333333"
            theme.page-bar-fg = "#444444"
            theme.page-bar-bg = "#555555"
            theme.page-bar-current-bg = "#666666"
            theme.post-header-fg = "#444444"
            theme.post-header-bg = "#555555"
            theme.post-upvoted-fg = "#666666"
            theme.post-upvoted-bg = "#777777"
            theme.post-heading-fg = "#888888"
            theme.post-heading-bg = "#999999"
            theme.post-body-fg = "#000000"
            theme.post-body-bg = "#111111"
            theme.cmd-plt-fg = "#222222"
            theme.cmd-plt-bg = "#333333"
        "##).unwrap();
        // let mut exp_config = Config::default();
        // exp_config.key_bindings.app_quit = "ABCDEFGHIJ".to_owned();
        assert_eq!(res_config, Config {
            key_bindings: HashMap::from([
                ("app_quit".to_owned(), "abcdefghi".to_owned())
            ]),
            tui: TuiPrefsDes {
                interface: InterfaceDes {
                    mouse_events_enable: false
                },
                theme: ThemeDes {
                    highlight_fg: "#222222".to_string(),
                    highlight_bg: "#333333".to_string(),
                    page_bar_fg: "#444444".to_string(),
                    page_bar_bg: "#555555".to_string(),
                    page_bar_current_bg: "#666666".to_string(),
                    post_header_fg: "#444444".to_string(),
                    post_header_bg: "#555555".to_string(),
                    post_upvoted_fg: "#666666".to_string(),
                    post_upvoted_bg: "#777777".to_string(),
                    post_heading_fg: "#888888".to_string(),
                    post_heading_bg: "#999999".to_string(),
                    post_body_fg: "#000000".to_string(),
                    post_body_bg: "#111111".to_string(),
                    cmd_plt_fg: "#222222".to_string(),
                    cmd_plt_bg: "#333333".to_string()
                }
            }
        });
    }
}
