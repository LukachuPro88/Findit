use crate::config::IGNORE_FILE_PATH;
use std::fs;

pub fn read_file() -> std::io::Result<Vec<String>> {
    let mutex = IGNORE_FILE_PATH.get_or_init(|| std::sync::Mutex::new(String::new()));
    let guard = mutex
        .lock()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let content = fs::read_to_string(&*guard)?;

    let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

    Ok(lines)
}
