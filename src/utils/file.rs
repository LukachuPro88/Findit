use crate::config::IGNORE_FILE_PATH;
use std::fs;
use std::path::Path;

pub fn read_ignore_file() -> std::io::Result<Vec<String>> {
    let mutex = IGNORE_FILE_PATH.get_or_init(|| std::sync::Mutex::new(std::path::PathBuf::new()));
    let guard = mutex
        .lock()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let content = fs::read_to_string(&*guard)?;
    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

pub fn read_file(file_path: &Path) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap_or_default()
        .split_whitespace()
        .map(|w| w.to_string())
        .collect()
}

pub fn write_file(file_path: &Path, content: &str) -> std::io::Result<()> {
    fs::write(file_path, content)
}
