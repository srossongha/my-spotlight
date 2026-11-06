pub mod hotkey;
pub mod plugins;
pub mod state;
pub mod window;

use tauri::{AppHandle, State};
use plugins::{Action, SearchResult};
use state::AppState;

#[tauri::command]
fn search(query: String, state: State<'_, AppState>) -> Vec<SearchResult> {
    plugins::dispatch_search(&query, &state)
}

#[tauri::command]
fn execute_action(action: Action, app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    plugins::execute_action(&action, &state)?;
    // Automatically hide spotlight window after action execution
    let _ = window::hide_window(&app);
    Ok(())
}

#[tauri::command]
fn hide_window(app: AppHandle) -> Result<(), String> {
    window::hide_window(&app)
}

#[tauri::command]
fn toggle_window(app: AppHandle) -> Result<(), String> {
    window::toggle_window(&app)
}

#[tauri::command]
fn reload_index(state: State<'_, AppState>) -> Result<usize, String> {
    Ok(state.reload_apps())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .setup(|app| {
            window::configure_window(app)?;
            hotkey::register_hotkey(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            search,
            execute_action,
            hide_window,
            toggle_window,
            reload_index
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
