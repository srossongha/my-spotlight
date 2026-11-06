use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use crate::state::AppEntry;
use super::{Action, SearchResult};

const SCAN_DIRS: &[&str] = &[
    "/Applications",
    "/System/Applications",
    "/System/Cryptexes/App/System/Applications",
    "/System/Library/CoreServices/Applications",
];

pub fn scan_all_apps() -> Vec<AppEntry> {
    let mut apps = Vec::new();
    let mut seen_paths = HashSet::new();

    // 1. Scan root app directories
    for dir_str in SCAN_DIRS {
        let path = Path::new(dir_str);
        if path.exists() && path.is_dir() {
            scan_dir(path, &mut apps, &mut seen_paths, 0);
        }
    }

    // 2. Scan user's ~/Applications directory
    if let Ok(home) = std::env::var("HOME") {
        let user_apps = PathBuf::from(home).join("Applications");
        if user_apps.exists() && user_apps.is_dir() {
            scan_dir(&user_apps, &mut apps, &mut seen_paths, 0);
        }
    }

    // Sort apps alphabetically by name
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    apps
}

fn scan_dir(
    dir: &Path,
    apps: &mut Vec<AppEntry>,
    seen_paths: &mut HashSet<String>,
    depth: usize,
) {
    if depth > 2 {
        return;
    }

    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let file_name = match entry.file_name().into_string() {
            Ok(n) => n,
            Err(_) => continue,
        };

        // Skip hidden files
        if file_name.starts_with('.') {
            continue;
        }

        if file_name.ends_with(".app") {
            let app_name = file_name.trim_end_matches(".app").to_string();
            let path_str = path.to_string_lossy().to_string();

            if seen_paths.insert(path_str.clone()) {
                apps.push(AppEntry {
                    name: app_name,
                    path: path_str,
                });
            }
        } else if path.is_dir() {
            // Recurse into subdirectories (like Utilities), but do NOT recurse into .app bundles
            scan_dir(&path, apps, seen_paths, depth + 1);
        }
    }
}

pub fn search_apps(query: &str, cached_apps: &[AppEntry]) -> Vec<SearchResult> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let matcher = SkimMatcherV2::default();
    let mut results = Vec::new();

    for app in cached_apps {
        if let Some(score) = matcher.fuzzy_match(&app.name, trimmed) {
            // Apply bonus if the app name starts with the query
            let starts_with_bonus = if app.name.to_lowercase().starts_with(&trimmed.to_lowercase()) {
                150
            } else {
                0
            };

            // Exact match bonus
            let exact_bonus = if app.name.eq_ignore_ascii_case(trimmed) {
                300
            } else {
                0
            };

            results.push(SearchResult {
                id: format!("app:{}", app.path),
                title: app.name.clone(),
                subtitle: app.path.clone(),
                icon: "app".to_string(),
                plugin: "apps".to_string(),
                action: Action::LaunchApp {
                    path: app.path.clone(),
                },
                score: score + starts_with_bonus + exact_bonus,
            });
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzzy_match_vsc() {
        let sample_apps = vec![
            AppEntry {
                name: "Visual Studio Code".to_string(),
                path: "/Applications/Visual Studio Code.app".to_string(),
            },
            AppEntry {
                name: "Safari".to_string(),
                path: "/Applications/Safari.app".to_string(),
            },
        ];

        let results = search_apps("vsc", &sample_apps);
        assert!(!results.is_empty());
        assert_eq!(results[0].title, "Visual Studio Code");
    }

    #[test]
    fn test_scan_finds_system_apps() {
        let apps = scan_all_apps();
        // macOS always has some system apps (Terminal, Safari, Calculator, etc.)
        assert!(!apps.is_empty(), "Should discover macOS applications");
    }
}
