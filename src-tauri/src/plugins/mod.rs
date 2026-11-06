pub mod apps;
pub mod calculator;
pub mod system;
pub mod web;

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::process::{Command, Stdio};
use crate::state::AppState;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    pub icon: String,
    pub plugin: String,
    pub action: Action,
    pub score: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum Action {
    LaunchApp { path: String },
    CopyToClipboard { text: String },
    RunSystemCommand { command: String },
    OpenUrl { url: String },
}

pub fn dispatch_search(query: &str, app_state: &AppState) -> Vec<SearchResult> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        // Return quick common system actions when query is empty
        return system::SYSTEM_COMMANDS
            .iter()
            .take(6)
            .map(|def| SearchResult {
                id: def.id.to_string(),
                title: def.title.to_string(),
                subtitle: def.subtitle.to_string(),
                icon: def.icon.to_string(),
                plugin: "system".to_string(),
                action: Action::RunSystemCommand {
                    command: def.command.to_string(),
                },
                score: 0,
            })
            .collect();
    }

    let mut results = Vec::new();

    // 1. Calculator plugin
    if let Some(calc_result) = calculator::evaluate_math(trimmed) {
        results.push(calc_result);
    }

    // 2. Apps plugin (reads from thread-safe cached apps)
    if let Ok(apps_lock) = app_state.apps.read() {
        let mut app_results = apps::search_apps(trimmed, &apps_lock);
        results.append(&mut app_results);
    }

    // 3. System commands plugin
    let mut sys_results = system::search_system_commands(trimmed);
    results.append(&mut sys_results);

    // 4. Web search fallback
    let mut web_results = web::search_web(trimmed);
    results.append(&mut web_results);

    // Sort by score descending
    results.sort_by(|a, b| b.score.cmp(&a.score));

    // Deduplicate by ID just in case
    let mut seen = std::collections::HashSet::new();
    results.retain(|item| seen.insert(item.id.clone()));

    results
}

pub fn execute_action(action: &Action, app_state: &AppState) -> Result<(), String> {
    match action {
        Action::LaunchApp { path } => {
            Command::new("open")
                .arg(path)
                .spawn()
                .map_err(|e| format!("Failed to launch app: {}", e))?;
            Ok(())
        }
        Action::CopyToClipboard { text } => {
            let mut child = Command::new("pbcopy")
                .stdin(Stdio::piped())
                .spawn()
                .map_err(|e| format!("Failed to spawn pbcopy: {}", e))?;

            if let Some(mut stdin) = child.stdin.take() {
                stdin
                    .write_all(text.as_bytes())
                    .map_err(|e| format!("Failed to write to clipboard: {}", e))?;
            }
            child
                .wait()
                .map_err(|e| format!("Failed waiting for pbcopy: {}", e))?;
            Ok(())
        }
        Action::RunSystemCommand { command } => {
            if command == "reload_index" {
                app_state.reload_apps();
                Ok(())
            } else {
                system::execute_system_command(command)
            }
        }
        Action::OpenUrl { url } => {
            Command::new("open")
                .arg(url)
                .spawn()
                .map_err(|e| format!("Failed to open URL: {}", e))?;
            Ok(())
        }
    }
}

