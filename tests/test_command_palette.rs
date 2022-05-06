use anyhow::Result;
use libnotcurses_sys::{ c_api, Nc, NcChannels, NcInput, NcKey, NcPlane, NcReceived, NcStyle };
use serial_test::serial;
use std::sync::{ Mutex, Arc };

use ravana::{
    jobs::config::load_config,
    tui::{ App, TuiPrefs },
    input::{ create_key_bindings_trie, handle_input, KeyCombination }
};

// -----------------------------------------------------------------------------------------------------------
// - Command palette should be at the top of App in CmdMode.
// -----------------------------------------------------------------------------------------------------------

#[test]
#[serial]
fn test_cmd_plt_pos() -> Result<()> {
    let mut config = load_config();
    config.tui.interface.mouse_events_enable = false; // Disable mouse events for GitHub Actions to work.
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let mut app = App::new(nc.clone(),
        TuiPrefs::gen_tui_prefs(&config.tui)?
    )?;
    app.dummy_render()?;

    app.enter_cmd()?;
    app.render().unwrap();

    // If command palette is at top it will return NULL for nc_above.
    // let nc_parent = unsafe { c_api::ncplane_parent(app.cmd_plt.plane as *mut NcPlane) };
    let nc_above = unsafe { c_api::ncplane_above(app.cmd_plt.plane as *mut NcPlane) };
    assert_eq!(nc_above, std::ptr::null::<NcPlane>() as *mut NcPlane);

    Ok(())
}

// -----------------------------------------------------------------------------------------------------------
// * Test if : produces cmd mode entry.
// * Test if text is correctly printed on cmd mode true.
// * Test if left-most char is : displayed on cmd mode.
// * Test if command palette becomes empty on exiting.
// -----------------------------------------------------------------------------------------------------------

#[test]
#[serial]
fn test_cmd_plt_text_display() -> Result<()> {
    let mut config = load_config();
    config.tui.interface.mouse_events_enable = false; // Disable mouse events for GitHub Actions to work.
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let nc_lock = nc.lock().unwrap();
    let (dim_y, _) = nc_lock.term_dim_yx();
    drop(nc_lock);

    let mut app = App::new(nc.clone(),
        TuiPrefs::gen_tui_prefs(&config.tui)?
    )?;
    app.dummy_render()?;

    let mut buffer: KeyCombination = KeyCombination::new();
    let mut cmd_mode: bool = false;
    let kbt = create_key_bindings_trie(&config.key_bindings)?;

    let mut ncin = NcInput::new(':');
    let ncr = NcReceived::Char(':');
    handle_input(&mut app, &mut buffer, &mut cmd_mode, &ncr, &mut ncin, &kbt)?;

    let eg_cmd_chars: Vec<char> = "example".chars().collect();
    for (_, v) in eg_cmd_chars.iter().enumerate() {
        let mut ncin = NcInput::new(*v);
        let ncr = NcReceived::Char(*v);
        handle_input(&mut app, &mut buffer, &mut cmd_mode, &ncr, &mut ncin, &kbt)?;
    }
    
    app.render().unwrap();

    let mut stylemask = NcStyle(0);
    let mut channels = NcChannels::new();

    // Test if left-most char is : on cmd mode.
    let mut nc_lock = nc.lock().unwrap();
    let s = nc_lock.at_yx(dim_y - 1, 0, &mut stylemask, &mut channels);
    drop(nc_lock);
    assert_eq!(s, Some(":".to_string()));

    // Test if text is correctly displsyed.
    for (i, v) in eg_cmd_chars.iter().enumerate() {
        let mut nc_lock = nc.lock().unwrap();
        let s = nc_lock.at_yx(dim_y - 1, (i + 1) as u32, &mut stylemask, &mut channels);
        drop(nc_lock);
        assert_eq!(s, Some(v.to_string()));
    }

    // On cmd mode exit, the command palette becomes empty.
    app.exit_cmd()?;
    for (i, v) in eg_cmd_chars.iter().enumerate() {
        let mut nc_lock = nc.lock().unwrap();
        let s = nc_lock.at_yx(dim_y - 1, (i + 1) as u32, &mut stylemask, &mut channels);
        drop(nc_lock);
        assert_eq!(s, Some(" ".to_string()));
    }
    Ok(())
}

// -----------------------------------------------------------------------------------------------------------
// * Test if : trigger cmd mode true.
// * Test if Esc/Enter leaves cmd mode.
// -----------------------------------------------------------------------------------------------------------

#[test]
#[serial]
fn test_cmd_mode_switching() -> Result<()> {
    let mut config = load_config();
    config.tui.interface.mouse_events_enable = false; // Disable mouse events for GitHub Actions to work.
    let nc = Arc::new(Mutex::new(unsafe { Nc::new()? }));

    let nc_lock = nc.lock().unwrap();
    let (dim_y, _) = nc_lock.term_dim_yx();
    drop(nc_lock);

    let mut app = App::new(nc.clone(),
        TuiPrefs::gen_tui_prefs(&config.tui)?
    )?;
    app.dummy_render()?;

    let mut buffer: KeyCombination = KeyCombination::new();
    let mut cmd_mode: bool = false;
    let kbt = create_key_bindings_trie(&config.key_bindings)?;


    // Enters cmd mode on :.
    let mut ncin = NcInput::new(':');
    let ncr = NcReceived::Char(':');
    handle_input(&mut app, &mut buffer, &mut cmd_mode, &ncr, &mut ncin, &kbt)?;
    app.render().unwrap();
    assert_eq!(cmd_mode, true);
    

    // Exits cmd mode on escape.
    let mut ncin = NcInput::new(':');
    let ncr = NcReceived::Event(NcKey::Esc);
    handle_input(&mut app, &mut buffer, &mut cmd_mode, &ncr, &mut ncin, &kbt)?;
    assert_eq!(cmd_mode, false);
    
    // Exits cmd mode on enter (and hopefully attempts executing).
    let mut ncin = NcInput::new(':');
    let ncr = NcReceived::Char(':');
    handle_input(&mut app, &mut buffer, &mut cmd_mode, &ncr, &mut ncin, &kbt)?;
    let mut ncin = NcInput::new(':');
    let ncr = NcReceived::Event(NcKey::Enter);
    handle_input(&mut app, &mut buffer, &mut cmd_mode, &ncr, &mut ncin, &kbt)?;
    assert_eq!(cmd_mode, false);

    Ok(())
}
