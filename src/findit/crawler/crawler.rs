use crate::utils::{file, logger};
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

pub fn traverse_words(file_path: &Path) -> Vec<String> {
    file::read_file(&file_path)
}
