//! handle the config file and bookmarks stored
//! in said config file

use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use sysinfo::{Pid, System};

const config_dir_path: &str = "~/.config/navigate/";

#[derive(Debug, Clone)]
pub struct Config {
    pub bookmarks: Vec<PathBuf>,
}

impl Config {
    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut config = Config {
            bookmarks: Vec::<PathBuf>::new(),
        };

        // stack.build_stack()?;
        if !config.bookmarks[0].is_dir() {
            config.bookmarks.remove(0);
        };

        Ok(config)
    }

    pub fn build_config(&mut self) -> Result<()> {
        let config_dir = match PathBuf::from_str(config_dir_path) {
            Ok(result) => result,
            Err(_) => return Err(Error::other("failed to create path object for config file")),
        };

        let mut bookmark_file = config_dir.clone();
        bookmark_file.push("bookmarks.conf");

        if !bookmark_file.is_file() {
            _ = File::create(bookmark_file.clone())?;
        }
        
        let mut bookmarks = fs::read_to_string(bookmark_file.clone())?;
        // TODO: parse bookmarks

        Ok(())
    }
}
