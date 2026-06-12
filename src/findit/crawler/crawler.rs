//! # crawler
//!
// traverses directories and adds each element to the list of elements to search.
use crate::utils::{file, logger};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn get_ignore_list() -> Vec<String> {
    crate::utils::file::read_ignore_file().unwrap_or_default()
}

fn should_ignore(path: &Path, ignore_list: &[String]) -> bool {
    let path_str = path.to_string_lossy();
    for pattern in ignore_list {
        if !pattern.is_empty() && path_str.contains(pattern) {
            return true;
        }
    }
    false
}

// Fast global check to block infinite system directories
fn is_system_dir(path: &Path) -> bool {
    path.starts_with("/proc")
        || path.starts_with("/sys")
        || path.starts_with("/dev")
        || path.starts_with("/run")
}

/// Traverses `start_path` and returns all files found.
///
/// Uses sequential iteration via [`walkdir`] for performance on large directory trees.
/// Skips any path that matches an entry in the ignore file via [`should_ignore`] or virtual system mounts.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use findit_rs::findit::crawler::crawler::traverse_files;
/// let files = traverse_files(Path::new("/home/user"));
/// ```
pub fn traverse_files(start_path: &Path) -> Vec<PathBuf> {
    let ignore_list = get_ignore_list();

    if should_ignore(start_path, &ignore_list) || is_system_dir(start_path) {
        logger::info(&format!("Ignoring '{}'", start_path.display()));
        return vec![];
    }

    let mut filenames = Vec::new();
    let mut it = WalkDir::new(start_path)
        .min_depth(1)
        .follow_links(false)
        .into_iter();

    while let Some(entry) = it.next() {
        if let Ok(entry) = entry {
            let path = entry.path();

            // PRUNE: Skip system loops before entering them
            if is_system_dir(path) {
                it.skip_current_dir();
                continue;
            }

            if should_ignore(path, &ignore_list) {
                logger::info(&format!("Ignoring '{}'", path.display()));
                if path.is_dir() {
                    it.skip_current_dir();
                }
            } else if path.is_file() {
                logger::info(&format!("File '{}', adding", path.display()));
                filenames.push(path.to_path_buf());
            } else {
                logger::info(&format!("Dir '{}', opening", path.display()));
            }
        }
    }

    filenames
}

/// Traverses `start_path` and returns all directories found.
///
/// Uses sequential iteration via [`walkdir`] for performance on large directory trees.
/// Skips any path that matches an entry in the ignore file via [`should_ignore`] or virtual system mounts.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use findit_rs::findit::crawler::crawler::traverse_dirs;
/// let dirs = traverse_dirs(Path::new("/home/user"));
/// ```
pub fn traverse_dirs(start_path: &Path) -> Vec<PathBuf> {
    let ignore_list = get_ignore_list();

    if should_ignore(start_path, &ignore_list) || is_system_dir(start_path) {
        logger::info(&format!("Ignoring '{}'", start_path.display()));
        return vec![];
    }

    let mut dirnames = Vec::new();
    let mut it = WalkDir::new(start_path)
        .min_depth(1)
        .follow_links(false)
        .into_iter();

    while let Some(entry) = it.next() {
        if let Ok(entry) = entry {
            let path = entry.path();

            // PRUNE: Skip system loops before entering them
            if is_system_dir(path) {
                it.skip_current_dir();
                continue;
            }

            if should_ignore(path, &ignore_list) {
                logger::info(&format!("Ignoring '{}'", path.display()));
                if path.is_dir() {
                    it.skip_current_dir();
                }
            } else if path.is_dir() {
                logger::info(&format!("Dir '{}', adding", path.display()));
                dirnames.push(path.to_path_buf());
            }
        }
    }

    dirnames
}

/// Sequentially traverses a directory tree to read all lines from non-ignored files,
/// formatting each line with its file path and line number for downstream filtering.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use findit_rs::findit::crawler::crawler::traverse_words;
/// let raw_lines = traverse_words(Path::new("/home/user/project"));
/// ```
pub fn traverse_words(dir_path: &Path) -> Vec<String> {
    let mut files = self::traverse_files(dir_path);
    files.sort();

    files
        .into_iter()
        .flat_map(|file_path| {
            let path_str = file_path.to_string_lossy().into_owned();
            let lines = file::read_file(&file_path);

            lines
                .into_iter()
                .enumerate()
                .map(move |(idx, line_text)| {
                    format!("{}:Line {}: {}", path_str, idx + 1, line_text.trim_end())
                })
                .collect::<Vec<String>>()
        })
        .collect()
}

// -------- TEST --------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_traverse_files_finds_files() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("main.rs"), "fn main() {}").unwrap();
        fs::write(dir.path().join("lib.rs"), "").unwrap();

        let files = traverse_files(dir.path());
        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_traverse_files_empty_dir() {
        let dir = tempdir().unwrap();
        let files = traverse_files(dir.path());
        assert!(files.is_empty());
    }

    #[test]
    fn test_traverse_files_recursive() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("src");
        fs::create_dir(&subdir).unwrap();
        fs::write(subdir.join("main.rs"), "").unwrap();

        let files = traverse_files(dir.path());
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("main.rs"));
    }

    #[test]
    fn test_traverse_dirs_finds_dirs() {
        let dir = tempdir().unwrap();
        fs::create_dir(dir.path().join("src")).unwrap();
        fs::create_dir(dir.path().join("tests")).unwrap();

        let dirs = traverse_dirs(dir.path());
        assert_eq!(dirs.len(), 2);
    }

    #[test]
    fn test_traverse_dirs_empty_dir() {
        let dir = tempdir().unwrap();
        let dirs = traverse_dirs(dir.path());
        assert!(dirs.is_empty());
    }

    #[test]
    fn test_traverse_dirs_recursive() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("src");
        fs::create_dir(&subdir).unwrap();
        fs::create_dir(subdir.join("nested")).unwrap();

        let dirs = traverse_dirs(dir.path());
        assert_eq!(dirs.len(), 2);
    }

    #[test]
    fn test_traverse_words_reads_lines() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello world\nfoo bar").unwrap();

        let words = traverse_words(dir.path());
        assert!(!words.is_empty());

        let target_match = format!("{}:Line 1: hello world", file.display());
        let found = words.iter().any(|line| line == &target_match);

        for w in &words {
            logger::debug(&format!("output {:?}", w));
        }

        assert!(found);
    }
    #[test]
    fn test_traverse_words_missing_file() {
        let result = traverse_words(std::path::Path::new("/nonexistent/directory"));
        assert!(result.is_empty());
    }
}
