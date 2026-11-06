use std::sync::RwLock;

#[derive(Debug, Clone)]
pub struct AppEntry {
    pub name: String,
    pub path: String,
}

pub struct AppState {
    pub apps: RwLock<Vec<AppEntry>>,
}

impl AppState {
    pub fn new() -> Self {
        let initial_apps = crate::plugins::apps::scan_all_apps();
        Self {
            apps: RwLock::new(initial_apps),
        }
    }

    pub fn reload_apps(&self) -> usize {
        let refreshed = crate::plugins::apps::scan_all_apps();
        let count = refreshed.len();
        if let Ok(mut lock) = self.apps.write() {
            *lock = refreshed;
        }
        count
    }
}

