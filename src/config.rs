#![allow(dead_code)]

//! handle the config file and bookmarks stored
//! in said config file

use crate::format::*;
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{Error, Result};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Config {
    conf_file: PathBuf,
    pub settings: Settings,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct Settings {
    pub general: GeneralSettings,
    pub format: FormatSettings,
    pub styles: StyleSettings,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct GeneralSettings {
    pub show_stack_on_push: bool,
    pub show_stack_on_pop: bool,
    pub show_stack_on_bookmark: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct FormatSettings {
    pub stack_separator: String,
    pub bookmarks_separator: String,
    pub align_separators: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
#[serde(default)]
pub struct StyleSettings {
    pub stack_number: String,
    pub stack_separator: String,
    pub stack_path: String,
    pub bookmarks_name: String,
    pub bookmarks_seperator: String,
    pub bookmarks_path: String,
}

impl Config {
    const CONFIG_FILE_NAME: &str = "navigate.conf";

    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut config = Config {
            conf_file: PathBuf::new(),
            settings: Settings {
                general: GeneralSettings {
                    show_stack_on_push: false,
                    show_stack_on_pop: false,
                    show_stack_on_bookmark: false,
                },
                format: FormatSettings {
                    bookmarks_separator: String::new(),
                    stack_separator: String::new(),
                    align_separators: false,
                },
                styles: StyleSettings {
                    stack_number: String::new(),
                    stack_separator: String::new(),
                    stack_path: String::new(),
                    bookmarks_name: String::new(),
                    bookmarks_seperator: String::new(),
                    bookmarks_path: String::new(),
                },
            },
        };
        // get configuration directory
        config.conf_file = match config_dir() {
            Some(value) => value,
            None => {
                return Err(Error::other(
                    "-- failed to retrieve configuration directory",
                ))
            }
        };
        // expand path to configuration file
        config
            .conf_file
            .push(format!("navigate/{}", Self::CONFIG_FILE_NAME));

        // parse configuration file and populate config struct
        let file_error = config.build_settings();
        config.set_default_settings()?;
        if file_error.is_err() {
            config.write_config_file()?;
        }
        config.parse_color_settings()?;

        Ok(config)
    }

    /// formats and prints config to string
    pub fn to_formatted_string(&self) -> Result<String> {
        let mut buffer = String::new();
        buffer = format!("{:#?}", self.settings);
        Ok(buffer)
    }

    /// reads and parses the configuration file
    fn build_settings(&mut self) -> Result<()> {
        if !self.conf_file.is_file() {
            return Err(Error::other(
                "-- config file `navigation.conf` does not exist",
            ));
        }
        let config_str = match fs::read_to_string(&self.conf_file) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };
        self.settings = match toml::from_str(&config_str) {
            Ok(value) => value,
            Err(error) => return Err(Error::other(error.to_string())),
        };
        Ok(())
    }

    /// sets defaults for settings not found in the configuration file
    fn set_default_settings(&mut self) -> Result<()> {
        let default_separator = " - ".to_owned();
        let default_number_color = "default".to_owned();
        let default_separator_color = "cyan".to_owned();
        let default_path_color = "default".to_owned();

        if self.settings.format.stack_separator.is_empty() {
            self.settings.format.stack_separator = default_separator.clone();
        }
        if self.settings.format.bookmarks_separator.is_empty() {
            self.settings.format.bookmarks_separator = default_separator.clone();
        }

        if self.settings.styles.stack_number.is_empty() {
            self.settings.styles.stack_number = default_number_color.clone();
        }
        if self.settings.styles.stack_separator.is_empty() {
            self.settings.styles.stack_separator = default_separator_color.clone();
        }
        if self.settings.styles.stack_path.is_empty() {
            self.settings.styles.stack_path = default_path_color.clone();
        }
        if self.settings.styles.bookmarks_name.is_empty() {
            self.settings.styles.bookmarks_name = default_number_color.clone();
        }
        if self.settings.styles.bookmarks_seperator.is_empty() {
            self.settings.styles.bookmarks_seperator = default_separator_color.clone();
        }
        if self.settings.styles.bookmarks_path.is_empty() {
            self.settings.styles.bookmarks_path = default_path_color.clone();
        }

        Ok(())
    }

    /// write configuration file to save changed settings
    pub fn write_config_file(&self) -> Result<()> {
        let conf_str = match toml::to_string(&self.settings) {
            Ok(value) => value,
            Err(error) => return Err(Error::other(error.to_string())),
        };
        fs::write(self.conf_file.clone(), conf_str)
    }

    /// convert color settings to ansi escape sequences
    pub fn parse_color_settings(&mut self) -> Result<()> {
        self.settings.styles.stack_number = parse_color(self.settings.styles.stack_number.clone())?;
        self.settings.styles.stack_separator =
            parse_color(self.settings.styles.stack_separator.clone())?;
        self.settings.styles.stack_path = parse_color(self.settings.styles.stack_path.clone())?;
        self.settings.styles.bookmarks_name =
            parse_color(self.settings.styles.bookmarks_name.clone())?;
        self.settings.styles.bookmarks_seperator =
            parse_color(self.settings.styles.bookmarks_seperator.clone())?;
        self.settings.styles.bookmarks_path =
            parse_color(self.settings.styles.bookmarks_path.clone())?;
        Ok(())
    }
}
