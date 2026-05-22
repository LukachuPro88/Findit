#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

pub static IGNORE_FILE_PATH: OnceLock<Mutex<PathBuf>> = OnceLock::new();

pub mod update;
