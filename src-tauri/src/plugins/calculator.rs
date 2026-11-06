use super::{Action, SearchResult};

pub fn evaluate_math(query: &str) -> Option<SearchResult> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return None;
    }

    // Quick filter: must contain at least one digit or a known math prefix/operator
    let has_digit = trimmed.chars().any(|c| c.is_ascii_digit());
    let has_math_op = trimmed.chars().any(|c| matches!(c, '+' | '-' | '*' | '/' | '^' | '%' | '=' | '×' | '÷'));
    let starts_with_math_fn = trimmed.starts_with("sqrt")
        || trimmed.starts_with("sin")
        || trimmed.starts_with("cos")
        || trimmed.starts_with("tan")
        || trimmed.starts_with("abs")
        || trimmed.starts_with("ln")
        || trimmed.starts_with("log");

    if !has_digit && !starts_with_math_fn {
        return None;
    }

    if !has_math_op && !starts_with_math_fn {
        // If it's just a plain number without operators (e.g. "123"), don't clutter search with a calculator card
        return None;
    }

    // Sanitize string for meval
    let sanitized = trimmed
        .replace('×', "*")
        .replace('÷', "/")
        .replace('=', "");

    let sanitized = sanitized.trim();
    if sanitized.is_empty() {
        return None;
    }

    match meval::eval_str(sanitized) {
        Ok(result) => {
            if result.is_nan() || result.is_infinite() {
                return None;
            }

            let formatted = if (result.fract() == 0.0) && (result.abs() < 1e15) {
                format!("{}", result as i64)
            } else {
                format!("{:.6}", result)
                    .trim_end_matches('0')
                    .trim_end_matches('.')
                    .to_string()
            };

            Some(SearchResult {
                id: format!("calc:{}", formatted),
                title: formatted.clone(),
                subtitle: format!("= {} (Press ↵ to copy)", formatted),
                icon: "calculator".to_string(),
                plugin: "calculator".to_string(),
                action: Action::CopyToClipboard { text: formatted },
                score: 100_000, // Top priority when user types a math expression
            })
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluates_basic_math() {
        let res = evaluate_math("24 * 15").expect("Should evaluate 24 * 15");
        assert_eq!(res.title, "360");
    }

    #[test]
    fn test_evaluates_division_and_decimals() {
        let res = evaluate_math("100 / 4").expect("Should evaluate 100 / 4");
        assert_eq!(res.title, "25");

        let res2 = evaluate_math("1 / 2").expect("Should evaluate 1 / 2");
        assert_eq!(res2.title, "0.5");
    }

    #[test]
    fn test_evaluates_math_functions() {
        let res = evaluate_math("sqrt(144)").expect("Should evaluate sqrt(144)");
        assert_eq!(res.title, "12");
    }

    #[test]
    fn test_ignores_non_math_queries() {
        assert!(evaluate_math("vsc").is_none());
        assert!(evaluate_math("safari").is_none());
        assert!(evaluate_math("").is_none());
        assert!(evaluate_math("123").is_none()); // Plain number without op
    }
}
