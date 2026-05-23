mod cli;
mod config;
mod findit;
mod gui;
mod utils;

#[cfg(target_os = "windows")]
fn get_platform_config_path() -> std::path::PathBuf {
    let mut path = std::path::PathBuf::from(std::env::var("APPDATA").unwrap_or_default());
    path.push("Findit");
    path.push("config");
    path
}

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
