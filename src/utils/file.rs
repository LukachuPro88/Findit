use crate::config::IGNORE_FILE_PATH;
use std::fs;

pub fn read_ignore_file() -> std::io::Result<Vec<String>> {
    let mutex = IGNORE_FILE_PATH.get_or_init(|| std::sync::Mutex::new(String::new()));
    let guard = mutex
        .lock()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let content = fs::read_to_string(&*guard)?;

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}

pub fn read_file(file_path: &str) -> Vec<String> {
    fs::read_to_string(file_path)
        .unwrap_or_default()
        .split_whitespace()
        .map(|w| w.to_string())
        .collect()
}

pub fn write_file(file_path: &str, content: &str) -> std::io::Result<()> {
    fs::write(file_path, content)
}
