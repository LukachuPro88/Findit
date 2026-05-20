use crate::utils::logger;

fn should_ignore(path: &str) -> bool {
    if let Ok(contents) = crate::utils::file::read_file() {
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
                let msg = format!("Ignoring '{}'", path_str);
                logger::info(&msg);
                continue;
            }

            if path.is_dir() {
                let msg = format!("Dir '{}', opening", path_str);
                logger::info(&msg);
                files.extend(traverse_files(&path_str));
            } else {
                let msg = format!("file '{}', adding", path_str);
                logger::info(&msg);

                files.push(path_str.into_owned());
            }
        }
    }

    files
}
