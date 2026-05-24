//! # filter
//!
//! Filters a list of [`std::path::PathBuf`] items based on a target string.

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
                        s.to_lowercase().ends_with(&target_lower)
                    } else {
                        s.ends_with(target_str)
                    }
                })
                .unwrap_or(false)
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
}
