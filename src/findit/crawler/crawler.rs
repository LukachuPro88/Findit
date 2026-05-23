//! # crawler
//!
//! Crawler traverses directories and adds each element to the list of elements to search.

use crate::utils::{file, logger};
use std::path::{Path, PathBuf};

/// Returns `true` if the given path matches any entry in the ignore file.
///
/// Reads ignore patterns from [`crate::config::IGNORE_FILE_PATH`].
/// Returns `false` if the ignore file cannot be read or no patterns match.
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

/// Recursively traverses `start_path` and returns all files found.
///
/// Skips any path that matches an entry in the ignore file via [`should_ignore`].
///
/// # Examples
///
/// ```no_run
/// let files = traverse_files(Path::new("/home/user"));
/// ```
pub fn traverse_files(start_path: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = Vec::new();
    if should_ignore(start_path) {
        return files;
    }
    if let Ok(entries) = std::fs::read_dir(start_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if should_ignore(&path) {
                logger::info(&format!("Ignoring '{}'", path.display()));
                continue;
            }
            if path.is_dir() {
                logger::info(&format!("Dir '{}', opening", path.display()));
                files.extend(traverse_files(&path));
            } else {
                logger::info(&format!("File '{}', adding", path.display()));
                files.push(path);
            }
        }
    }
    files
}

/// Recursively traverses `start_path` and returns all directories found.
///
/// Skips any path that matches an entry in the ignore file via [`should_ignore`].
///
/// # Examples
///
/// ```no_run
/// let dirs = traverse_dirs(Path::new("/home/user"));
/// ```
pub fn traverse_dirs(start_path: &Path) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(start_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if should_ignore(&path) {
                logger::info(&format!("Ignoring '{}'", path.display()));
                continue;
            }
            if path.is_dir() {
                logger::info(&format!("Dir '{}', adding", path.display()));
                dirs.extend(traverse_dirs(&path));
                dirs.push(path);
            }
        }
    }
    dirs
}

/// Reads the contents of a file and returns each line as a string.
///
/// # Examples
///
/// ```no_run
/// let lines = traverse_words(Path::new("/home/user/file.txt"));
/// ```
pub fn traverse_words(file_path: &Path) -> Vec<String> {
    file::read_file(&file_path)
}
