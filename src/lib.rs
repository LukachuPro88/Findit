pub mod config;
pub mod findit;
pub mod utils;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

/// Returns the Windows path to the findit configuration file.
#[cfg(target_os = "windows")]
fn get_platform_config_path() -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(std::env::var("APPDATA").unwrap_or_default());
    path.push("Findit");
    path.push("config");
    path
}

/// Returns the Unix path to the findit configuration file.
#[cfg(not(target_os = "windows"))]
fn get_platform_config_path() -> std::path::PathBuf {
    #[allow(deprecated)]
    let mut path = std::env::home_dir()
        .unwrap_or_else(|| std::path::PathBuf::from(std::env::var("HOME").unwrap_or_default()));
    path.push(".config");
    path.push("Findit");
    path.push("config");
    path
}

/// FFI Entrypoint for Go Cgo binding
#[unsafe(no_mangle)]
pub extern "C" fn findit_crawl(
    c_start_path: *const c_char,
    c_target: *const c_char,
    mode: *const c_char,
) {
    if c_start_path.is_null() || c_target.is_null() || mode.is_null() {
        return;
    }

    let start_path = unsafe { CStr::from_ptr(c_start_path) }.to_string_lossy();
    let target = unsafe { CStr::from_ptr(c_target) }.to_string_lossy();
    let search_mode = unsafe { CStr::from_ptr(mode) }.to_string_lossy();

    crate::utils::set_level(crate::utils::Level::SUCCESS);
    let config_path = get_platform_config_path();

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        let path_str = content.trim();
        if !path_str.is_empty() {
            crate::config::update::update_ignore_file_path(Path::new(path_str));
        }
    }

    let start_path_str: &str = &start_path;
    let target_str: &str = &target;

    match search_mode.as_ref() {
        "dir" => {
            let raw_results =
                crate::findit::crawler::crawler::traverse_dirs(Path::new(start_path_str));

            let filtered = crate::findit::filter::filter::filter(raw_results, target_str);
            for path in filtered {
                println!("[SUCCESS]   ~ {}", path.display());
            }
        }
        "file" => {
            let raw_results =
                crate::findit::crawler::crawler::traverse_files(Path::new(start_path_str));

            let filtered = crate::findit::filter::filter::filter(raw_results, target_str);
            for path in filtered {
                println!("[SUCCESS]   ~ {}", path.display());
            }
        }
        "word" => {
            let raw_results =
                crate::findit::crawler::crawler::traverse_words(Path::new(start_path_str));

            let filtered = crate::findit::filter::filter::filter_words(raw_results, target_str);
            for match_str in filtered {
                println!("[SUCCESS]   ~ {}", match_str);
            }
        }
        _ => println!("[ERROR] Invalid search mode passed from Go interface"),
    }
}
