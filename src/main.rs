mod cli;
mod config;
mod findit;
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

fn main() {
    utils::set_level(utils::Level::SUCCESS);

    let config_path = get_platform_config_path();

    if let Ok(content) = std::fs::read_to_string(&config_path) {
        let path_str = content.trim();
        if !path_str.is_empty() {
            config::update::update_ignore_file_path(std::path::Path::new(path_str));
        }
    }

    cli::cli::main_cli();
}
