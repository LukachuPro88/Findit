#![allow(dead_code)]

use std::sync::{Mutex, OnceLock};

pub static IGNORE_FILE_PATH: OnceLock<Mutex<String>> = OnceLock::new();

pub mod update;
