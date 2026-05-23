//! # logger
//!
//! Formatted console logging using [`Level`] filtering.

#![allow(dead_code)]

use super::{Level, color, should_log};

/// Logs a debug message if [`Level::DEBUG`] is enabled.
pub fn debug(message: &str) {
    if should_log(Level::DEBUG) {
        println!(
            "{}{}[{:?}]{} {}",
            color::BOLD,
            color::BLUE,
            Level::DEBUG,
            color::RESET,
            message
        );
    }
}

/// Logs an info message if [`Level::INFO`] is enabled.
pub fn info(message: &str) {
    if should_log(Level::INFO) {
        println!(
            "{}{}[{:?}]{} {}",
            color::BOLD,
            color::RESET,
            Level::INFO,
            color::RESET,
            message
        );
    }
}

/// Logs a success message if [`Level::SUCCESS`] is enabled.
pub fn success(message: &str) {
    if should_log(Level::SUCCESS) {
        println!(
            "{}{}[{:?}]{} {}",
            color::BOLD,
            color::GREEN,
            Level::SUCCESS,
            color::RESET,
            message
        );
    }
}

/// Logs an error message if [`Level::ERROR`] is enabled.
pub fn error(message: &str) {
    if should_log(Level::ERROR) {
        println!(
            "{}{}[{:?}]{} {}",
            color::BOLD,
            color::RED,
            Level::ERROR,
            color::RESET,
            message
        );
    }
}
