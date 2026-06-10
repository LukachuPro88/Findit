//! # file
//!
//! Utilities for reading and writing files.

use crate::config::IGNORE_FILE_PATH;
use std::fs;
use std::path::Path;

/// Reads the ignore file and returns each line as a string.
///
/// The file path is read from [`IGNORE_FILE_PATH`].
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the mutex is poisoned or the file cannot be read.
pub fn read_ignore_file() -> std::io::Result<Vec<String>> {
    let mutex = IGNORE_FILE_PATH.get_or_init(|| std::sync::Mutex::new(std::path::PathBuf::new()));
    let guard = mutex
        .lock()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let content = fs::read_to_string(&*guard)?;
    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

/// Reads a file and returns each whitespace-separated word as a string.
///
/// Returns an empty vector if the file cannot be read.
pub fn read_file(file_path: &Path) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap_or_default()
        .lines()
        .map(|l| l.to_string())
        .collect()
}
/// Writes `content` to the given file path, creating or overwriting it.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the file cannot be written.
pub fn write_file(file_path: &Path, content: &str) -> std::io::Result<()> {
    fs::write(file_path, content)
}

// -------- TEST --------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_read_file_returns_words() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, "hello world foo bar").unwrap();
        let words = read_file(&file);
        assert_eq!(words, vec!["hello world foo bar"]);
    }

    #[test]
    fn test_read_file_missing_returns_empty() {
        let result = read_file(std::path::Path::new("/nonexistent/file.txt"));
        assert!(result.is_empty());
    }

    #[test]
    fn test_write_file_creates_file() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("output.txt");
        write_file(&file, "hello").unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "hello");
    }

    #[test]
    fn test_write_file_overwrites() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("output.txt");
        write_file(&file, "first").unwrap();
        write_file(&file, "second").unwrap();
        assert_eq!(fs::read_to_string(&file).unwrap(), "second");
    }
}
