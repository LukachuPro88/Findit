use crate::utils::{file, logger};

fn should_ignore(path: &str) -> bool {
    if let Ok(contents) = crate::utils::file::read_ignore_file() {
        for line in contents.iter() {
            if !line.is_empty() && path.contains(line) {
                return true;
            }
        }
    }
    false
}

pub fn traverse_files(start_path: &str) -> Vec<String> {
    let mut files: Vec<String> = Vec::new();
    if should_ignore(start_path) {
        return files;
    }
    if let Ok(entries) = std::fs::read_dir(start_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy();
            if should_ignore(&path_str) {
                logger::info(&format!("Ignoring '{}'", path_str));
                continue;
            }
            if path.is_dir() {
                logger::info(&format!("Dir '{}', opening", path_str));
                files.extend(traverse_files(&path_str));
            } else {
                logger::info(&format!("File '{}', adding", path_str));
                files.push(path_str.into_owned());
            }
        }
    }
    files
}

pub fn traverse_dirs(start_path: &str) -> Vec<String> {
    let mut dirs: Vec<String> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(start_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            let path_str = path.to_string_lossy();
            if should_ignore(&path_str) {
                logger::info(&format!("Ignoring '{}'", path_str));
                continue;
            }
            if path.is_dir() {
                logger::info(&format!("Dir '{}', adding", path_str));
                dirs.push(path_str.clone().into_owned());
                dirs.extend(traverse_dirs(path_str.as_ref()));
            }
        }
    }
    dirs
}

pub fn traverse_words(file_path: &str) -> Vec<String> {
    file::read_file(file_path)
}
