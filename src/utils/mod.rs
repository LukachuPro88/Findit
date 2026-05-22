#![allow(dead_code)]

use std::collections::HashSet;
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
static DISABLED_LEVELS: Mutex<Option<HashSet<u8>>> = Mutex::new(None);

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

pub fn disable_level(level: Level) {
    let mut disabled = DISABLED_LEVELS.lock().unwrap();
    disabled
        .get_or_insert_with(HashSet::new)
        .insert(level as u8);
}

pub fn enable_level(level: Level) {
    let mut disabled = DISABLED_LEVELS.lock().unwrap();
    if let Some(set) = disabled.as_mut() {
        set.remove(&(level as u8));
    }
}

pub(crate) fn should_log(level: Level) -> bool {
    let current = CURRENT_LEVEL.lock().unwrap();
    if *current == Level::NONE {
        return false;
    }
    let level_u8 = level as u8;
    let current_u8 = *current as u8;
    if level_u8 < current_u8 {
        return false;
    }
    let disabled = DISABLED_LEVELS.lock().unwrap();
    if let Some(set) = disabled.as_ref() {
        return !set.contains(&level_u8);
    }
    true
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
