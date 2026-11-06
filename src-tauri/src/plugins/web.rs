use super::{Action, SearchResult};

fn url_encode(input: &str) -> String {
    let mut encoded = String::with_capacity(input.len());
    for b in input.bytes() {
        match b {
            b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(b as char);
            }
            b' ' => encoded.push('+'),
            _ => {
                encoded.push_str(&format!("%{:02X}", b));
            }
        }
    }
    encoded
}

pub fn search_web(query: &str) -> Vec<SearchResult> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Vec::new();
    }

    let encoded = url_encode(trimmed);

    vec![
        SearchResult {
            id: format!("web:google:{}", trimmed),
            title: format!("Search Google for \"{}\"", trimmed),
            subtitle: "Open search results in your default browser".to_string(),
            icon: "google".to_string(),
            plugin: "web".to_string(),
            action: Action::OpenUrl {
                url: format!("https://www.google.com/search?q={}", encoded),
            },
            score: 15,
        },
        SearchResult {
            id: format!("web:ddg:{}", trimmed),
            title: format!("Search DuckDuckGo for \"{}\"", trimmed),
            subtitle: "Open privacy search in your default browser".to_string(),
            icon: "duckduckgo".to_string(),
            plugin: "web".to_string(),
            action: Action::OpenUrl {
                url: format!("https://duckduckgo.com/?q={}", encoded),
            },
            score: 10,
        },
    ]
}

