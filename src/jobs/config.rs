use anyhow::Result;
use log::{ error, info, warn };
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};

use crate::def::app::CONFIG_DIR_PATHS;
use super::util::config::Config;

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
    error!("Unable to open config file. Returning default config.");
    Config::default()
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ Config, deserialize_toml };
    use crate::jobs::{ InterfaceDes, ThemeDes, TuiPrefsDes };

    // Test if deserialize_toml deserializes toml proper.
    #[test]
    fn test_deserialize_toml() {
        let res_config: Config = deserialize_toml(r##"
            [key-bindings]
            app_quit = "abcdefghi"

            [tui]
            interface.mouse-events-enable = false

            theme.highlight-fg = "#222222"
            theme.highlight-bg = "#333333"
            theme.post-header-fg = "#444444"
            theme.post-header-bg = "#555555"
            theme.post-upvoted-fg = "#666666"
            theme.post-upvoted-bg = "#777777"
            theme.post-heading-fg = "#888888"
            theme.post-heading-bg = "#999999"
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
                    post_header_fg: "#444444".to_string(),
                    post_header_bg: "#555555".to_string(),
                    post_upvoted_fg: "#666666".to_string(),
                    post_upvoted_bg: "#777777".to_string(),
                    post_heading_fg: "#888888".to_string(),
                    post_heading_bg: "#999999".to_string()
                }
            }
        });
    }
}
