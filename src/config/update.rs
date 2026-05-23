//! # update
//!
//! Handles updating the project configurations.

use crate::config::IGNORE_FILE_PATH;
use crate::utils::file;
use crate::utils::logger;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Returns the Windows configuration directory for findit.
///
/// Resolves to `%APPDATA%\Findit`, falling back to an empty path
/// if `APPDATA` is not set.
#[cfg(target_os = "windows")]
fn get_platform_config_dir() -> PathBuf {
    let mut path = PathBuf::from(std::env::var("APPDATA").unwrap_or_default());
    path.push("Findit");
    path
}

/// Returns the Unix configuration directory for findit.
///
/// Resolves to `$HOME/.config/Findit`, falling back to an empty path
/// if `$HOME` is not set.
#[cfg(not(target_os = "windows"))]
fn get_platform_config_dir() -> PathBuf {
    #[allow(deprecated)]
    let mut path = std::env::home_dir()
        .unwrap_or_else(|| PathBuf::from(std::env::var("HOME").unwrap_or_default()));
    path.push(".config");
    path.push("Findit");
    path
}

/// Returns the home directory for the current user.
///
/// Falls back to an empty path if the home directory cannot be determined.
fn get_home_dir() -> PathBuf {
    #[allow(deprecated)]
    std::env::home_dir().unwrap_or_else(|| PathBuf::from(std::env::var("HOME").unwrap_or_default()))
}

/// Updates the ignore file path to the given path.
///
/// Expands `~` to the home directory if the path starts with `~`.
/// Logs an error if the [`IGNORE_FILE_PATH`] mutex is poisoned.
pub fn update_ignore_file_path(path: &Path) {
    let mut expanded = PathBuf::new();

    if path.starts_with("~") {
        expanded.push(get_home_dir());
        if let Ok(stripped) = path.strip_prefix("~") {
            expanded.push(stripped);
        }
    } else {
        expanded.push(path);
    }

    let mutex_ref = IGNORE_FILE_PATH.get_or_init(|| Mutex::new(expanded.clone()));
    match mutex_ref.lock() {
        Ok(mut guard) => {
            *guard = expanded.clone();
            logger::debug(&format!(
                "Ignore file path updated to: {}",
                expanded.display()
            ));
        }
        Err(_) => {
            logger::error("Failed to lock IGNORE_FILE_PATH: Mutex is poisoned");
        }
    }
}

/// Persists the ignore file path to the platform-specific configuration directory.
///
/// Writes the current [`IGNORE_FILE_PATH`] to:
/// - Windows: `%APPDATA%\Findit\config`
/// - Unix: `$HOME/.config/Findit/config`
///
/// Logs an error if the directory cannot be created or the file cannot be written.
pub fn persist_ignore_file_path() {
    let mutex = IGNORE_FILE_PATH.get_or_init(|| Mutex::new(PathBuf::new()));
    let guard = mutex.lock().unwrap();

    let config_dir = get_platform_config_dir();

    let mut config_path = config_dir.clone();
    config_path.push("config");

    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        logger::error(&format!("Failed to create config directory: {}", e));
        return;
    }

    if let Err(e) = file::write_file(&config_path, &guard.to_string_lossy()) {
        logger::error(&format!("Failed to persist ignore file path: {}", e));
    }
}
