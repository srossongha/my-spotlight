use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutState};

pub fn register_hotkey(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let shortcut = "alt+space".parse::<Shortcut>()?;
    let app_handle = app.handle().clone();

    app.global_shortcut().on_shortcut(shortcut, move |_app, _shortcut, event| {
        if event.state() == ShortcutState::Pressed {
            let _ = crate::window::toggle_window(&app_handle);
        }
    })?;

    Ok(())
}

