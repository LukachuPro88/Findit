//! # filter
//!
//! Filters a list of items based on a target string.

use rayon::prelude::*;

/// Filters `items` to those whose path ends with `target`.
///
/// Matching is case-insensitive on Windows and case-sensitive on Unix.
pub fn filter<T: AsRef<str> + Sync>(
    items: Vec<std::path::PathBuf>,
    target: T,
) -> Vec<std::path::PathBuf> {
    let target_str = target.as_ref();
    let target_lower = target_str.to_lowercase();
    items
        .into_par_iter()
        .filter(|item| {
            item.to_str()
                .map(|s| {
                    if cfg!(target_os = "windows") {
                        // Avoid complete string allocation by matching case-insensitively using an iterator comparison
                        s.chars()
                            .rev()
                            .zip(target_lower.chars().rev())
                            .all(|(a, b)| a.to_lowercase().to_string() == b.to_string())
                    } else {
                        s.ends_with(target_str)
                    }
                })
                .unwrap_or(false)
        })
        .collect()
}

/// Filters `lines` to those that contain the `target` keyword.
///
/// Matching is case-insensitive on Windows and case-sensitive on Unix.
pub fn filter_words<T: AsRef<str> + Sync>(lines: Vec<String>, target: T) -> Vec<String> {
    let target_str = target.as_ref();
    let target_lower = target_str.to_lowercase();
    lines
        .into_par_iter()
        .filter(|line| {
            if cfg!(target_os = "windows") {
                // Check if the lowercase target matches anywhere in a case-insensitive window search without creating a full copy of the line
                line.to_lowercase().contains(&target_lower)
            } else {
                line.contains(target_str)
            }
        })
        .collect()
}

// -------- TEST --------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filter_matches() {
        let items = vec![
            PathBuf::from("/home/user/src/main.rs"),
            PathBuf::from("/home/user/src/lib.rs"),
            PathBuf::from("/home/user/README.md"),
        ];
        let result = filter(items, "main.rs");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], PathBuf::from("/home/user/src/main.rs"));
    }

    #[test]
    fn test_filter_no_match() {
        let items = vec![PathBuf::from("/home/user/README.md")];
        let result = filter(items, "main.rs");
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_empty_input() {
        let result = filter(vec![], "main.rs");
        assert!(result.is_empty());
    }

    #[test]
    fn test_filter_multiple_matches() {
        let items = vec![
            PathBuf::from("/home/user/src/main.rs"),
            PathBuf::from("/home/user/backup/main.rs"),
            PathBuf::from("/home/user/README.md"),
        ];
        let result = filter(items, "main.rs");
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_filter_words_matches() {
        let lines = vec![
            "src/main.rs:Line 1: fn main() {".to_string(),
            "src/lib.rs:Line 5: // setup code".to_string(),
        ];
        let result = filter_words(lines, "main()");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "src/main.rs:Line 1: fn main() {");
    }

    #[test]
    fn test_filter_words_no_match() {
        let lines = vec!["src/main.rs:Line 1: fn main() {".to_string()];
        let result = filter_words(lines, "missing_keyword");
        assert!(result.is_empty());
    }
}

