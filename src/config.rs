#![allow(dead_code)]

//! handle the config file and bookmarks stored
//! in said config file

use dirs::config_dir;
use std::fs;
use std::io::{Error, Result};
use config_parser::*;

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct Config {
    #[nested_config]
    pub general: GeneralSettings,
    #[nested_config]
    pub format: FormatSettings,
    #[nested_config]
    pub styles: StyleSettings,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct GeneralSettings {
    #[default_value(false)]
    pub show_stack_on_push: bool,
    #[default_value(false)]
    pub show_stack_on_pop: bool,
    #[default_value(false)]
    pub show_books_on_bookmark: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct FormatSettings {
    #[default_value(true)]
    pub align_separators: bool,
    #[default_value(" - ")]
    pub stack_separator: String,
    #[default_value(false)]
    pub stack_home_as_tilde: bool,
    #[default_value(" - ")]
    pub bookmarks_separator: String,
    #[default_value(false)]
    pub book_home_as_tilde: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct StyleSettings {
    #[style_config]
    #[default_value("yellow, italic")]
    pub warning_style: String,
    #[style_config]
    #[default_value("red, bold")]
    pub error_style: String,
    #[style_config]
    #[default_value("default")]
    pub stack_number_style: String,
    #[style_config]
    #[default_value("cyan")]
    pub stack_separator_style: String,
    #[style_config]
    #[default_value("default")]
    pub stack_path_style: String,
    #[style_config]
    #[default_value("green")]
    pub stack_punct_style: String,
    #[style_config]
    #[default_value("default")]
    pub bookmarks_name_style: String,
    #[style_config]
    #[default_value("cyan")]
    pub bookmarks_seperator_style: String,
    #[style_config]
    #[default_value("default")]
    pub bookmarks_path_style: String,
    #[style_config]
    #[default_value("green")]
    pub bookmarks_punct_style: String,
}

impl Config {
    const CONFIG_FILE_NAME: &str = "navigate.conf";

    /// generates and populates a new instance of Config
    pub fn new(styles_as_ansi_sequences: bool) -> Result<Self> {
        let mut config: Config = Self::default();
        // get configuration directory
        let mut conf_file = match config_dir() {
            Some(value) => value,
            None => {
                return Err(Error::other(
                    "-- failed to retrieve configuration directory",
                ))
            }
        };
        // expand path to configuration file
        conf_file.push(format!("navigate/{}", Self::CONFIG_FILE_NAME));

        // parse configuration file and populate config struct
        if conf_file.is_file() {
            let config_str = match fs::read_to_string(&conf_file) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            _ = config.parse_from_string(&config_str);
        } else {
            // TODO: write default configuration
        }

        if styles_as_ansi_sequences {
            config.to_ansi_sequences()?;
        }

        Ok(config)
    }

    /// formats and prints config to string
    pub fn to_formatted_string(&self) -> Result<String> {
        Ok(format!("{:#?}", self))
    }

}
