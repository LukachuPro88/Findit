#![allow(dead_code)]

use super::{Level, color, should_log};

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
