#![allow(dead_code)]

//! handle the config file and bookmarks stored
//! in said config file

use dirs::config_dir;
use std::fs;
use std::io::{Error, Result};
use std::path::PathBuf;
use config_parser::*;

#[derive(Debug, Clone)]
pub struct Config {
    conf_file: PathBuf,
    pub settings: Settings,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct Settings {
    #[nested_config]
    pub general: GeneralSettings,
    #[nested_config]
    pub format: FormatSettings,
    #[nested_config]
    pub styles: StyleSettings,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct GeneralSettings {
    pub show_stack_on_push: bool,
    pub show_stack_on_pop: bool,
    pub show_stack_on_bookmark: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct FormatSettings {
    pub stack_separator: String,
    pub bookmarks_separator: String,
    pub align_separators: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct StyleSettings {
    #[style_config]
    pub stack_number_style: String,
    #[style_config]
    pub stack_separator_style: String,
    #[style_config]
    pub stack_path_style: String,
    #[style_config]
    pub bookmarks_name_style: String,
    #[style_config]
    pub bookmarks_seperator_style: String,
    #[style_config]
    pub bookmarks_path_style: String,
}

impl Config {
    const CONFIG_FILE_NAME: &str = "navigate.conf";

    /// generates and populates a new instance of Config
    pub fn new(styles_as_ansi_sequences: bool) -> Result<Self> {
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
                    stack_number_style: String::new(),
                    stack_separator_style: String::new(),
                    stack_path_style: String::new(),
                    bookmarks_name_style: String::new(),
                    bookmarks_seperator_style: String::new(),
                    bookmarks_path_style: String::new(),
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
        if config.conf_file.is_file() {
            let config_str = match fs::read_to_string(&config.conf_file) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            _ = config.settings.parse_from_string(&config_str);
        } else {
            // TODO: write default configuration
        }

        // TODO: ALSDKJFJFJASDkk
        //config.set_default_settings()?;
        //config.parse_color_settings()?;

        if styles_as_ansi_sequences {
            config.settings.to_ansi_sequences()?;
        }

        Ok(config)
    }

    /// formats and prints config to string
    pub fn to_formatted_string(&self) -> Result<String> {
        Ok(format!("{:#?}", self.settings))
    }

    /// reads and parses the configuration file
    fn build_settings(&mut self) -> Result<()> {
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

        if self.settings.styles.stack_number_style.is_empty() {
            self.settings.styles.stack_number_style = default_number_color.clone();
        }
        if self.settings.styles.stack_separator_style.is_empty() {
            self.settings.styles.stack_separator_style = default_separator_color.clone();
        }
        if self.settings.styles.stack_path_style.is_empty() {
            self.settings.styles.stack_path_style = default_path_color.clone();
        }
        if self.settings.styles.bookmarks_name_style.is_empty() {
            self.settings.styles.bookmarks_name_style = default_number_color.clone();
        }
        if self.settings.styles.bookmarks_seperator_style.is_empty() {
            self.settings.styles.bookmarks_seperator_style = default_separator_color.clone();
        }
        if self.settings.styles.bookmarks_path_style.is_empty() {
            self.settings.styles.bookmarks_path_style = default_path_color.clone();
        }

        Ok(())
    }
}
