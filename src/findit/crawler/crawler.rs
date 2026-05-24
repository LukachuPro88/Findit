//! # crawler
//!
//! Crawler traverses directories and adds each element to the list of elements to search.
use crate::utils::{file, logger};
use rayon::prelude::*;
use std::path::{Path, PathBuf};

fn should_ignore(path: &Path) -> bool {
    if let Ok(contents) = crate::utils::file::read_ignore_file() {
        for line in contents.iter() {
            if !line.is_empty() && path.to_string_lossy().contains(line.as_str()) {
                return true;
            }
        }
    }
    false
}

/// Traverses `start_path` and returns all files found.
///
/// Uses parallel iteration via [`rayon`] for performance on large directory trees.
/// Skips any path that matches an entry in the ignore file via [`should_ignore`].
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use findit_rs::findit::crawler::crawler::traverse_files;
/// let files = traverse_files(Path::new("/home/user"));
/// ```
pub fn traverse_files(start_path: &Path) -> Vec<PathBuf> {
    if should_ignore(start_path) {
        return vec![];
    }
    let entries: Vec<_> = match std::fs::read_dir(start_path) {
        Ok(e) => e.flatten().collect(),
        Err(_) => return vec![],
    };
    entries
        .into_par_iter()
        .flat_map(|entry| {
            let path = entry.path();
            if should_ignore(&path) {
                logger::info(&format!("Ignoring '{}'", path.display()));
                vec![]
            } else if path.is_dir() {
                logger::info(&format!("Dir '{}', opening", path.display()));
                traverse_files(&path)
            } else {
                logger::info(&format!("File '{}', adding", path.display()));
                vec![path]
            }
        })
        .collect()
}

/// Traverses `start_path` and returns all directories found.
///
/// Uses parallel iteration via [`rayon`] for performance on large directory trees.
/// Skips any path that matches an entry in the ignore file via [`should_ignore`].
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use findit_rs::findit::crawler::crawler::traverse_dirs;
/// let dirs = traverse_dirs(Path::new("/home/user"));
/// ```
pub fn traverse_dirs(start_path: &Path) -> Vec<PathBuf> {
    let entries: Vec<_> = match std::fs::read_dir(start_path) {
        Ok(e) => e.flatten().collect(),
        Err(_) => return vec![],
    };
    entries
        .into_par_iter()
        .flat_map(|entry| {
            let path = entry.path();
            if should_ignore(&path) {
                logger::info(&format!("Ignoring '{}'", path.display()));
                vec![]
            } else if path.is_dir() {
                logger::info(&format!("Dir '{}', adding", path.display()));
                let mut dirs = traverse_dirs(&path);
                dirs.push(path);
                dirs
            } else {
                vec![]
            }
        })
        .collect()
}

/// Reads the contents of a file and returns each line as a string.
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use findit_rs::findit::crawler::crawler::traverse_words;
/// let lines = traverse_words(Path::new("/home/user/file.txt"));
/// ```
pub fn traverse_words(file_path: &Path) -> Vec<String> {
    file::read_file(file_path)
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

        let words = traverse_words(&file);
        assert!(!words.is_empty());
        assert!(words.contains(&"hello".to_string()));
    }

    #[test]
    fn test_traverse_words_missing_file() {
        let result = traverse_words(std::path::Path::new("/nonexistent/file.txt"));
        assert!(result.is_empty());
    }
}
