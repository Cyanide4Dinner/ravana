use anyhow::{ anyhow, Result };
use libnotcurses_sys::Nc;
use log::error;
use ravana_reddit_api::{ api::RedditClient, auth::Scope };
use std::sync::{ Mutex, Arc };
use std::{
    fs::File,
    path::Path
};

use crate::{
    input::{ listen, create_key_bindings_trie },
    jobs::config::{ get_session_file_path, load_config, load_session, save_session },
    tools::{ log_err_desc_ret, log_err_ret },
    tui::{ App, TuiPrefs, val_tui_prefs_des },
};

const CLIENT_ID: &str = "znL1Gj0B5rXi4gyCdmCBwg";
const REDIRECT_URL: &str = "http://localhost:5555";

// -----------------------------------------------------------------------------------------------------------
// * Main loop.
// * Input --> Process --> State --> TUI --> Input.
// -----------------------------------------------------------------------------------------------------------
pub fn ravana() -> Result<()> {
    let session_file_path = get_session_file_path()?;
    let mut session = load_session(&mut File::open(
            Path::new(&format!("{}{}",
            session_file_path,
            "/Session.toml"))
        )?);

    //TODO: Generate user agent from system, user information.
    let mut reddit_client: RedditClient = RedditClient::new(CLIENT_ID, REDIRECT_URL, None, "Ravana/Reddit")?;

    if session.refresh_token.is_empty() {
        let (oauth_url, csrf_tok) = reddit_client.oauth_url(vec![Scope::Read]);
        println!("Oauth URL: {}", oauth_url);
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                reddit_client.oauth_flow(csrf_tok, "<html><h1>Success</h1></html>".to_string()).await
            })?;
        session.refresh_token = "xyz".to_string();
        save_session(&session, &mut File::create(
            Path::new(&format!("{}{}",
                session_file_path, "/Session.toml"))
        )?)?;
        println!("Refresh token saved.");
        return Ok(());
    }

    let config = load_config();

    // Instantiating Nc instance.
    let nc = Arc::new(Mutex::new(unsafe { log_err_desc_ret!(Nc::new(), "Failed to instantiate Nc.")? }));

    // Validate config
    {
        if !val_tui_prefs_des(&config.tui) { 
            return log_err_ret!(Err(anyhow!("Invalid TUI format in config.")));
        } 
    }

    let mut app = App::new(nc.clone(),
        log_err_desc_ret!(TuiPrefs::gen_tui_prefs(&config.tui),
            "Failed to generate TUI prefs"
        )?,
        reddit_client
    )?;
    app.dummy_render()?;
    app.render().unwrap();

    let kbt = log_err_desc_ret!(create_key_bindings_trie(&config.key_bindings), "Failed to create KB trie")?;

    listen(nc, kbt, &mut app).unwrap();

    Ok(())
}
