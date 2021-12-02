pub mod config {
    use serde::Deserialize;

    #[derive(PartialEq, Eq)]
    #[derive(Deserialize, Debug)]
    #[serde(rename_all(deserialize = "kebab-case"))]
    #[serde(default)]
    pub struct Config {
        pub key_bindings: KeyBindings
    }

    #[derive(PartialEq, Eq)]
    #[derive(Deserialize, Debug)]
    #[serde(default)]
    pub struct KeyBindings {
        pub app_quit: String,
    }

    impl Default for Config {
        fn default() -> Config {
            Config {
                key_bindings: KeyBindings::default()
            }
        }
    }

    impl Default for KeyBindings {
        fn default() -> KeyBindings {
            KeyBindings {
                app_quit: "ZZ".to_string()
            }
        }
    }
}

pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum JobError {
       #[error("Failed to complete job: {0}")] 
        JobFailError(String)
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
}
