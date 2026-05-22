use crate::findit::crawler::crawler;
use crate::findit::filter::filter;
use crate::utils::logger;
use std::env;
use std::path::PathBuf;

enum Command {
    Dir {
        start_path: PathBuf,
        name: String,
        verbose: bool,
    },
    File {
        start_path: PathBuf,
        name: String,
        verbose: bool,
    },
    Word {
        file_path: PathBuf,
        word: String,
        verbose: bool,
    },
    Ignore {
        file_path: PathBuf,
    },
}

fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = env::args().collect();
    match args.as_slice() {
        [_, flag, arg1, arg2, verbose] if verbose == "--verbose" => match flag.as_str() {
            "--dir" => Ok(Command::Dir {
                start_path: PathBuf::from(arg1),
                name: arg2.clone(),
                verbose: true,
            }),
            "--file" => Ok(Command::File {
                start_path: PathBuf::from(arg1),
                name: arg2.clone(),
                verbose: true,
            }),
            "--word" => Ok(Command::Word {
                file_path: PathBuf::from(arg1),
                word: arg2.clone(),
                verbose: true,
            }),
            _ => Err(format!("Unknown flag: {}\n{}", flag, print_usage())),
        },
        [_, flag, arg1, arg2] => match flag.as_str() {
            "--dir" => Ok(Command::Dir {
                start_path: PathBuf::from(arg1),
                name: arg2.clone(),
                verbose: false,
            }),
            "--file" => Ok(Command::File {
                start_path: PathBuf::from(arg1),
                name: arg2.clone(),
                verbose: false,
            }),
            "--word" => Ok(Command::Word {
                file_path: PathBuf::from(arg1),
                word: arg2.clone(),
                verbose: false,
            }),
            _ => Err(format!("Unknown flag: {}\n{}", flag, print_usage())),
        },
        [_, flag, arg1] => match flag.as_str() {
            "--ignore" => Ok(Command::Ignore {
                file_path: PathBuf::from(arg1),
            }),
            _ => Err(format!("Unknown flag: {}\n{}", flag, print_usage())),
        },
        _ => Err(print_usage()),
    }
}

fn set_verbose(verbose: bool) {
    if verbose {
        crate::utils::set_level(crate::utils::Level::INFO);
        crate::utils::disable_level(crate::utils::Level::DEBUG);
    }
}

pub fn main_cli() {
    match parse_args() {
        Ok(command) => match command {
            Command::Dir {
                start_path,
                name,
                verbose,
            } => {
                set_verbose(verbose);
                let dirs = crawler::traverse_dirs(&start_path);
                let matching_dirs = filter::filter(dirs, &name);
                if matching_dirs.is_empty() {
                    logger::error(&format!("Directory '{}' not found", name));
                } else {
                    logger::success(&format!(
                        "Found {} matching directories",
                        matching_dirs.len()
                    ));
                    for dir in &matching_dirs {
                        logger::success(&format!("  ~ {}", dir.display()));
                    }
                }
            }
            Command::File {
                start_path,
                name,
                verbose,
            } => {
                set_verbose(verbose);
                let files = crawler::traverse_files(&start_path);
                let matching_files = filter::filter(files, &name);
                if matching_files.is_empty() {
                    logger::error(&format!("File '{}' not found", name));
                } else {
                    logger::success(&format!("Found {} matching files", matching_files.len()));
                    for file in &matching_files {
                        logger::success(&format!("  ~ {}", file.display()));
                    }
                }
            }
            Command::Word {
                file_path,
                word,
                verbose,
            } => {
                set_verbose(verbose);
                if !file_path.exists() {
                    logger::error(&format!("File '{}' not found", file_path.display()));
                    return;
                }
                let lines = crawler::traverse_words(&file_path);
                let matching_lines: Vec<(usize, &String)> = lines
                    .iter()
                    .enumerate()
                    .filter(|(_, line)| line.contains(word.as_str()))
                    .collect();
                if matching_lines.is_empty() {
                    logger::error(&format!("Word '{}' not found", word));
                } else {
                    logger::success(&format!("Word '{}' found:", word));
                    for (i, line) in &matching_lines {
                        logger::success(&format!("  ~ {}: {}", i + 1, line));
                    }
                }
            }
            Command::Ignore { file_path } => {
                crate::config::update::update_ignore_file_path(&file_path);
                crate::config::update::persist_ignore_file_path();
                logger::success(&format!("Ignore file updated to: {}", file_path.display()));
            }
        },
        Err(e) => println!("{}", e),
    }
}

fn print_usage() -> String {
    format!(
        "Usage:\n  findit --dir    <start_path> <name>  [--verbose]\n  findit --file   <start_path> <name>  [--verbose]\n  findit --word   <file_path>  <word>  [--verbose]\n  findit --ignore <file_path>"
    )
}
