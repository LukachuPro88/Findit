mod cli;
mod config;
mod findit;
mod utils;

fn main() {
    utils::set_level(utils::Level::SUCCESS);
    let home = std::env::var("HOME").unwrap_or_default();
    let config_path = format!("{}/.config/Findit/config", home);
    if let Ok(path) = std::fs::read_to_string(&config_path) {
        let path = path.trim();
        if !path.is_empty() {
            config::update::update_ignore_file_path(path);
        }
    }
    cli::cli::main_cli();
}
