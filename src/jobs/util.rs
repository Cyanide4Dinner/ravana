pub mod config {
    use serde::Deserialize;
    use std::collections::HashMap;

    use crate::input::DEFAULT_KEY_BINDINGS;

    // Theme deserialized.
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all(deserialize = "kebab-case")/*,default*/)]
    pub struct ThemeDes {
        pub highlight_fg: String,
        pub highlight_bg: String,
        pub post_header_fg: String,
        pub post_header_bg: String,
        pub post_upvoted_fg: String,
        pub post_upvoted_bg: String
    }

    // TUI Prefs deserialized.
    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all(deserialize = "kebab-case")/*,default*/)]
    pub struct TuiPrefsDes {
        pub theme: ThemeDes
    }

    #[derive(Deserialize, Debug, PartialEq, Eq)]
    #[serde(rename_all(deserialize = "kebab-case"))]
    pub struct Config {
        pub key_bindings: HashMap<String, String>,
        pub tui: TuiPrefsDes 
    }
    
    //TODO: Resolve default configuration from default Config.toml directly - for cases where
    // Config.toml is not found
    impl Default for Config {
        fn default() -> Config {
            let mut default_key_bindings: HashMap<String, String> = HashMap::new();
            for (&key, &value) in DEFAULT_KEY_BINDINGS.entries() {
                default_key_bindings.insert(key.to_owned(), value.to_owned());
            }
            Config {
                key_bindings: default_key_bindings,
                tui: TuiPrefsDes {
                    theme: ThemeDes {
                        highlight_fg: "#111111".to_string(),
                        highlight_bg: "#111111".to_string(),
                        post_header_fg: "#111111".to_string(), 
                        post_header_bg: "#111111".to_string(),
                        post_upvoted_fg: "#111111".to_string(),
                        post_upvoted_bg: "#111111".to_string()
                    }
                }
            }
        }
    }
}
