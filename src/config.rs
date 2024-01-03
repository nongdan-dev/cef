use std::path::PathBuf;

use crate::ffi::*;

pub struct Config {
    pub url: String,
    pub width: i32,
    pub height: i32,
    pub raw_output: PathBuf,
    pub mp4_output: PathBuf,
    pub raw_handler: bool,
    pub log_level: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            url: Default::default(),
            width: 1920,
            height: 1080,
            raw_output: Default::default(),
            mp4_output: Default::default(),
            raw_handler: Default::default(),
            log_level: cef_log_severity_t_LOGSEVERITY_ERROR,
        }
    }
}
