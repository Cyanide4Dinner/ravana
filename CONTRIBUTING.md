## Add a new property in Config.toml

- Fields in Config.toml must be in kebab-case and the corresponding fields in structs must be in snakecase.
- Add the property in Config.toml.
- Add in relevant structs in jobs/util.rs.
- Add defaults in Default impl of Config in jobs/util.rs.
- Fix tests in jobs/config.rs.
- Add relevant fields to TuiPrefs and fix impl TuiPrefs in tui/util.rs (if applicable).
- Add validation in val_tui_prefs_des in tui/util.rs (if applicable).
