//! # filter
//!
//! Filters a list of [`std::path::PathBuf`] items based on a target string.

/// Filters `items` to those whose path ends with `target`.
///
/// Matching is case-insensitive on Windows and case-sensitive on Unix.
pub fn filter<T: AsRef<str>>(items: Vec<std::path::PathBuf>, target: T) -> Vec<std::path::PathBuf> {
    let target_str = target.as_ref();

    items
        .into_iter()
        .filter(|item| {
            item.to_str()
                .map(|s| {
                    if cfg!(target_os = "windows") {
                        s.to_lowercase().ends_with(&target_str.to_lowercase())
                    } else {
                        s.ends_with(target_str)
                    }
                })
                .unwrap_or(false)
        })
        .collect()
}
