use anyhow::{ Result, /*Context*/ };
use log::{ error, warn };
use std::{
    fs::File,
    io::prelude::*,
    path::Path
};

use crate::def::app::CONFIG_DIR_PATHS;
use super::util::{
    config::{ Config },
    // error::JobError
};

// TODO Integration test: Default struct and default Config.toml correspond.
pub async fn load_config() -> Config {
    for path in CONFIG_DIR_PATHS {
        match File::open(Path::new(&format!("{}{}", path, "/Config.toml"))) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    if let Ok(config) = deserialize_toml(&contents) {
                        return config
                    }
                }
                else { warn!("Unable to open file: {}", path); }
            }
            Err(e) => { warn!("Skipping opening file: {}, due to: {:?}", path, e); }
        }
    }
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
    use crate::jobs::util::config::Config;

    use super::deserialize_toml;

    #[test]
    fn test_deserialize_toml() {
        let res_config: Config = deserialize_toml(r#"
            [key-bindings]
            app_quit = "ABCDEFGHIJ"
        "#).unwrap();
        let mut exp_config = Config::default();
        exp_config.key_bindings.app_quit = "ABCDEFGHIJ".to_owned();
        assert_eq!(res_config, exp_config);
        exp_config.key_bindings.app_quit = "ABC".to_owned();
        assert_ne!(res_config, exp_config);
    }
}
