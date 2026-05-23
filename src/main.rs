//! # findit-rs
//!
//! A fast and lightweight file, directory, and word search tool.
//!
//! Supports both a CLI and a GUI interface via the `--g` or `gui` flag.

mod cli;
mod config;
mod findit;
mod gui;
mod utils;

/// Returns the Windows path to the findit configuration file.
///
/// Resolves to `%APPDATA%\Findit\config`, falling back to an empty path
/// if `APPDATA` is not set.
#[cfg(target_os = "windows")]
fn get_platform_config_path() -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(std::env::var("APPDATA").unwrap_or_default());
    path.push("Findit");
    path.push("config");
    path
}

/// Returns the Unix path to the findit configuration file.
///
/// Resolves to `$HOME/.config/Findit/config`, falling back to an empty path
/// if `$HOME` is not set.
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

/// Entry point for findit-rs.
///
/// Initialises logging, loads the ignore file path from the platform-specific
/// configuration file, then launches either the GUI or CLI based on the arguments.
///
/// # GUI
///
/// Pass `--g` or `gui` as an argument to launch the GUI.
///
/// # Errors
///
/// Returns an [`eframe::Error`] if the GUI fails to launch.
fn main() -> Result<(), eframe::Error> {
    utils::set_level(utils::Level::SUCCESS);

    let config_path = get_platform_config_path();

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        let path_str = content.trim();
        if !path_str.is_empty() {
            config::update::update_ignore_file_path(std::path::Path::new(path_str));
        }
    }

    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|arg| arg == "--g") || args.iter().any(|arg| arg == "gui") {
        let native_options = eframe::NativeOptions::default();

        return eframe::run_native(
            "Findit Desktop Workspace",
            native_options,
            Box::new(|_cc| Ok(Box::new(gui::gui::Gui::default()))),
        );
    }

    cli::cli::main_cli();

    Ok(())
}
