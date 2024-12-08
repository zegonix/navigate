//! handle the config file and bookmarks stored
//! in said config file

use std::env::var;
use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;

use crate::{RESET_SEQ, STYLES};

#[derive(Debug, Clone)]
pub struct Config {
    conf_dir: PathBuf,
    pub settings: Settings,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Settings {
    pub general: GeneralSettings,
    pub styles: StyleSettings,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeneralSettings {
    pub show_stack_on_push: bool,
    pub show_stack_on_pop: bool,
    pub show_stack_on_bookmark: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FormatSettings {
    pub stack_separator: String,
    pub bookmarks_separator: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StyleSettings {
    pub stack_number: String,
    pub stack_separator: String,
    pub stack_path: String,
    pub bookmarks_key: String,
    pub bookmarks_seperator: String,
    pub bookmarks_path: String,
    pub reset: String,
}

impl Config {
    const CONFIG_FILE_NAME: &str = "navigate.conf";

    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut config = Config {
            conf_dir: PathBuf::new(),
            settings: Settings {
                general: GeneralSettings {
                    show_stack_on_push: false,
                    show_stack_on_pop: false,
                    show_stack_on_bookmark: false,
                },
                styles: StyleSettings {
                    stack_number: "".to_owned(),
                    stack_separator: "".to_owned(),
                    stack_path: "".to_owned(),
                    bookmarks_key: "".to_owned(),
                    bookmarks_seperator: "".to_owned(),
                    bookmarks_path: "".to_owned(),
                    reset: RESET_SEQ.to_owned(),
                },
            },
        };
        // get home directory path
        let home_dir = match var("HOME") {
            Ok(value) => value,
            Err(error) => return Err(Error::other(error.to_string())),
        };
        // create PathBuf object from home dir path
        config.conf_dir = match PathBuf::from_str(&home_dir) {
            Ok(value) => value,
            Err(error) => return Err(Error::other(error.to_string())),
        };
        // expand home directory path to get configuration directory path
        config.build_config()?;

        Ok(config)
    }

    /// reads and parses the configuration file
    fn build_config(&mut self) -> Result<()> {
        Ok(())
    }
}
