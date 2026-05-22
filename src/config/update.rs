use crate::config::IGNORE_FILE_PATH;
use crate::utils::file;
use crate::utils::logger;
use std::sync::Mutex;

pub fn update_ignore_file_path(path: &str) {
    let expanded = if path.starts_with("~/") {
        let home = std::env::var("HOME").unwrap_or_default();
        path.replacen("~/", &format!("{}/", home), 1)
    } else {
        path.to_string()
    };
    let mutex_ref = IGNORE_FILE_PATH.get_or_init(|| Mutex::new(expanded.clone()));
    match mutex_ref.lock() {
        Ok(mut guard) => {
            *guard = expanded.clone();
            logger::debug(&format!("Ignore file path updated to: {}", expanded));
        }
        Err(_) => {
            logger::error("Failed to lock IGNORE_FILE_PATH: Mutex is poisoned");
        }
    }
}

pub fn persist_ignore_file_path() {
    let mutex = IGNORE_FILE_PATH.get_or_init(|| std::sync::Mutex::new(String::new()));
    let guard = mutex.lock().unwrap();
    let home = std::env::var("HOME").unwrap_or_default();
    let config_dir = format!("{}/.config/Findit", home);
    let config_path = format!("{}/config", config_dir);
    if let Err(e) = std::fs::create_dir_all(&config_dir) {
        logger::error(&format!("Failed to create config directory: {}", e));
        return;
    }
    if let Err(e) = file::write_file(&config_path, &*guard) {
        logger::error(&format!("Failed to persist ignore file path: {}", e));
    }
}
