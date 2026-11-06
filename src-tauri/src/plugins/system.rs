use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use super::{Action, SearchResult};

#[derive(Debug, Clone)]
pub struct SystemCommandDef {
    pub id: &'static str,
    pub title: &'static str,
    pub subtitle: &'static str,
    pub icon: &'static str,
    pub command: &'static str,
    pub keywords: &'static [&'static str],
}

pub const SYSTEM_COMMANDS: &[SystemCommandDef] = &[
    SystemCommandDef {
        id: "system:lock",
        title: "Lock Screen",
        subtitle: "Lock macOS session and show login screen",
        icon: "lock",
        command: "lock",
        keywords: &["lock", "screen", "logout", "cgsession"],
    },
    SystemCommandDef {
        id: "system:sleep",
        title: "Sleep",
        subtitle: "Put Mac into low-power sleep mode",
        icon: "sleep",
        command: "sleep",
        keywords: &["sleep", "suspend", "standby"],
    },
    SystemCommandDef {
        id: "system:restart",
        title: "Restart",
        subtitle: "Restart the Mac computer",
        icon: "restart",
        command: "restart",
        keywords: &["restart", "reboot"],
    },
    SystemCommandDef {
        id: "system:shutdown",
        title: "Shut Down",
        subtitle: "Power off the Mac computer completely",
        icon: "shutdown",
        command: "shutdown",
        keywords: &["shutdown", "shut", "power off", "turn off"],
    },
    SystemCommandDef {
        id: "system:trash",
        title: "Empty Trash",
        subtitle: "Permanently delete items currently in the Trash",
        icon: "trash",
        command: "empty_trash",
        keywords: &["trash", "empty", "bin", "clean", "garbage"],
    },
    SystemCommandDef {
        id: "system:dark_mode",
        title: "Toggle Dark Mode",
        subtitle: "Switch appearance between Light and Dark mode",
        icon: "dark_mode",
        command: "toggle_dark_mode",
        keywords: &["dark", "light", "appearance", "theme", "mode"],
    },
    SystemCommandDef {
        id: "system:reload",
        title: "Reload Index",
        subtitle: "Re-scan all installed applications and refresh cache",
        icon: "reload",
        command: "reload_index",
        keywords: &["reload", "refresh", "index", "scan", "rescan", "cache"],
    },
];

pub fn search_system_commands(query: &str) -> Vec<SearchResult> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let matcher = SkimMatcherV2::default();
    let mut results = Vec::new();

    for def in SYSTEM_COMMANDS {
        let mut best_score: Option<i64> = matcher.fuzzy_match(def.title, trimmed);

        for kw in def.keywords {
            if let Some(kw_score) = matcher.fuzzy_match(kw, trimmed) {
                best_score = Some(best_score.map_or(kw_score, |s| s.max(kw_score)));
            }
        }

        if let Some(score) = best_score {
            results.push(SearchResult {
                id: def.id.to_string(),
                title: def.title.to_string(),
                subtitle: def.subtitle.to_string(),
                icon: def.icon.to_string(),
                plugin: "system".to_string(),
                action: Action::RunSystemCommand {
                    command: def.command.to_string(),
                },
                score: score + 50, // Slight boost for system commands
            });
        }
    }

    results
}

pub fn execute_system_command(command: &str) -> Result<(), String> {
    match command {
        "lock" => {
            let cgsession = std::process::Command::new("/System/Library/CoreServices/Menu Extras/User.menu/Contents/Resources/CGSession")
                .arg("-suspend")
                .spawn();

            if cgsession.is_err() {
                std::process::Command::new("open")
                    .args(["-a", "ScreenSaverEngine"])
                    .spawn()
                    .map_err(|e| e.to_string())?;
            }
            Ok(())
        }
        "sleep" => {
            std::process::Command::new("pmset")
                .arg("sleepnow")
                .spawn()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "restart" => {
            std::process::Command::new("osascript")
                .args(["-e", "tell application \"System Events\" to restart"])
                .spawn()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "shutdown" => {
            std::process::Command::new("osascript")
                .args(["-e", "tell application \"System Events\" to shut down"])
                .spawn()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "empty_trash" => {
            std::process::Command::new("osascript")
                .args(["-e", "tell application \"Finder\" to empty trash"])
                .spawn()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        "toggle_dark_mode" => {
            std::process::Command::new("osascript")
                .args(["-e", "tell application \"System Events\" to tell appearance preferences to set dark mode to not dark mode"])
                .spawn()
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        _ => Err(format!("Unknown system command: {}", command)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_lock() {
        let results = search_system_commands("lock");
        assert!(!results.is_empty());
        assert_eq!(results[0].title, "Lock Screen");
    }

    #[test]
    fn test_search_dark_mode() {
        let results = search_system_commands("dark");
        assert!(!results.is_empty());
        assert_eq!(results[0].title, "Toggle Dark Mode");
    }
}
