//! # cli
//!
//! This module provides the CLI interface for the findit tool.
//!
//! Handles parsing and validating command line arguments.
//!
//! # Supported Commands
//!
//! - `--dir <start_path> <name>` — Search for a directory by name.
//! - `--file <start_path> <name>` — Search for a file by name.
//! - `--word <file_path> <word>` — Search for a word in a file.
//! - `--ignore <file_path>` — Set the ignore file path.
//!
//! All commands except `--ignore` support an optional `--verbose` flag.

use crate::findit::crawler::crawler;
use crate::findit::filter::filter;
use crate::utils::logger;
use std::env;
use std::path::PathBuf;

/// Shared options for filesystem search commands.
struct SearchOptions {
    /// The starting path for the search.
    start_path: PathBuf,
    /// The name to search for.
    name: String,
    /// Enable verbose output.
    verbose: bool,
}

/// Represents the command to be executed by the CLI.
enum Command {
    /// Directory search command.
    Dir {
        /// Common search options.
        options: SearchOptions,
    },
    /// File search command.
    File {
        /// Common search options.
        options: SearchOptions,
    },
    /// Word search command.
    Word {
        /// The file path to search in.
        file_path: PathBuf,
        /// The word to search for.
        word: String,
        /// Enable verbose output.
        verbose: bool,
    },
    /// Ignore command.
    Ignore {
        /// The file path to the ignore file.
        file_path: PathBuf,
    },
}

/// Parses a slice of command-line arguments into a [`Command`].
///
/// # Errors
///
/// Returns `Err` with a usage message if:
/// - An unknown flag is provided.
/// - The argument count doesn't match any known pattern.
fn parse_args_from(args: &[String]) -> Result<Command, String> {
    match args {
        [_, flag, arg1, arg2, verbose] if verbose == "--verbose" => match flag.as_str() {
            "--dir" => Ok(Command::Dir {
                options: SearchOptions {
                    start_path: PathBuf::from(arg1),
                    name: arg2.clone(),
                    verbose: true,
                },
            }),
            "--file" => Ok(Command::File {
                options: SearchOptions {
                    start_path: PathBuf::from(arg1),
                    name: arg2.clone(),
                    verbose: true,
                },
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
                options: SearchOptions {
                    start_path: PathBuf::from(arg1),
                    name: arg2.clone(),
                    verbose: false,
                },
            }),
            "--file" => Ok(Command::File {
                options: SearchOptions {
                    start_path: PathBuf::from(arg1),
                    name: arg2.clone(),
                    verbose: false,
                },
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

/// Parses command-line arguments from [`std::env::args`] into a [`Command`].
///
/// # Errors
///
/// Returns `Err` with a usage message if parsing fails.
/// See [`parse_args_from`] for details.
fn parse_args() -> Result<Command, String> {
    let args: Vec<String> = env::args().collect();
    parse_args_from(&args)
}

/// Enables `INFO` logging when `verbose` is `true`, otherwise sets level to
/// [`crate::utils::Level::NONE`].
fn set_verbose(verbose: bool) {
    if verbose {
        crate::utils::set_level(crate::utils::Level::INFO);
        crate::utils::disable_level(crate::utils::Level::DEBUG);
    }
}

/// The main CLI entry point.
///
/// Parses command-line arguments via [`parse_args`] and dispatches to the
/// appropriate handler for [`Command::Dir`], [`Command::File`],
/// [`Command::Word`], or [`Command::Ignore`].
///
/// Prints a usage message to stdout if argument parsing fails.
pub fn main_cli() {
    match parse_args() {
        Ok(command) => match command {
            Command::Dir { options } => {
                set_verbose(options.verbose);
                let dirs = crawler::traverse_dirs(&options.start_path);
                let matching_dirs = filter::filter(dirs, &options.name);
                if matching_dirs.is_empty() {
                    logger::error(&format!("Directory '{}' not found", options.name));
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
            Command::File { options } => {
                set_verbose(options.verbose);
                let files = crawler::traverse_files(&options.start_path);
                let matching_files = filter::filter(files, &options.name);
                if matching_files.is_empty() {
                    logger::error(&format!("File '{}' not found", options.name));
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

/// Returns the usage message string.
fn print_usage() -> String {
    format!(
        "Usage:\n  findit --dir    <start_path> <name>  [--verbose]\n  findit --file   <start_path> <name>  [--verbose]\n  findit --word   <file_path>  <word>  [--verbose]\n  findit --ignore <file_path>"
    )
}

// -------- TEST --------

#[cfg(test)]
mod tests {
    use super::*;

    fn args(s: &[&str]) -> Vec<String> {
        s.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_parse_dir() {
        let result = parse_args_from(&args(&["findit", "--dir", "/home", "src"]));
        assert!(matches!(result, Ok(Command::Dir { .. })));
    }

    #[test]
    fn test_parse_file() {
        let result = parse_args_from(&args(&["findit", "--file", "/home", "main.rs"]));
        assert!(matches!(result, Ok(Command::File { .. })));
    }

    #[test]
    fn test_parse_word() {
        let result = parse_args_from(&args(&["findit", "--word", "/home/file.txt", "fn"]));
        assert!(matches!(result, Ok(Command::Word { .. })));
    }

    #[test]
    fn test_parse_ignore() {
        let result = parse_args_from(&args(&["findit", "--ignore", "/home/.finditignore"]));
        assert!(matches!(result, Ok(Command::Ignore { .. })));
    }

    #[test]
    fn test_parse_verbose() {
        let result = parse_args_from(&args(&["findit", "--dir", "/home", "src", "--verbose"]));
        assert!(matches!(
            result,
            Ok(Command::Dir {
                options: SearchOptions { verbose: true, .. }
            })
        ));
    }

    #[test]
    fn test_parse_unknown_flag() {
        let result = parse_args_from(&args(&["findit", "--unknown", "/home", "src"]));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_no_args() {
        let result = parse_args_from(&args(&["findit"]));
        assert!(result.is_err());
    }
}
