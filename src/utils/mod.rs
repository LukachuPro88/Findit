#![allow(dead_code)]

use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    DEBUG,
    INFO,
    SUCCESS,
    WARNING,
    ERROR,
    NONE,
}

static CURRENT_LEVEL: Mutex<Level> = Mutex::new(Level::DEBUG);
static PREVIOUS_LEVEL: Mutex<Level> = Mutex::new(Level::DEBUG);

pub fn toggle() {
    let mut current = CURRENT_LEVEL.lock().unwrap();
    let mut previous = PREVIOUS_LEVEL.lock().unwrap();

    if *current == Level::NONE {
        *current = *previous;
    } else {
        *previous = *current;
        *current = Level::NONE;
    }
}

pub fn set_level(level: Level) {
    let mut current = CURRENT_LEVEL.lock().unwrap();
    let mut previous = PREVIOUS_LEVEL.lock().unwrap();

    *previous = *current;
    *current = level;
}

pub(crate) fn should_log(level: Level) -> bool {
    let current = CURRENT_LEVEL.lock().unwrap();
    if *current == Level::NONE {
        return false;
    }

    level as u8 >= *current as u8
}

pub mod color {
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const BOLD: &str = "\x1b[1m";
    pub const RESET: &str = "\x1b[0m";
}
pub mod file;
pub mod logger;
