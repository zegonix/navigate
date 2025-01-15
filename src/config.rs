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
    pub show_stack_on_push: bool,
    pub show_stack_on_pop: bool,
    pub show_books_on_bookmark: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct FormatSettings {
    pub align_separators: bool,
    pub stack_separator: String,
    pub stack_home_as_tilde: bool,
    pub bookmarks_separator: String,
    pub book_home_as_tilde: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct StyleSettings {
    #[style_config]
    pub warning_style: String,
    #[style_config]
    pub error_style: String,
    #[style_config]
    pub stack_number_style: String,
    #[style_config]
    pub stack_separator_style: String,
    #[style_config]
    pub stack_path_style: String,
    #[style_config]
    pub stack_punct_style: String,
    #[style_config]
    pub bookmarks_name_style: String,
    #[style_config]
    pub bookmarks_seperator_style: String,
    #[style_config]
    pub bookmarks_path_style: String,
    #[style_config]
    pub bookmarks_punct_style: String,
}

impl Config {
    const CONFIG_FILE_NAME: &str = "navigate.conf";

    /// generates and populates a new instance of Config
    pub fn new(styles_as_ansi_sequences: bool) -> Result<Self> {
        let mut config = Config {
            general: GeneralSettings {
                show_stack_on_push: false,
                show_stack_on_pop: false,
                show_books_on_bookmark: false,
            },
            format: FormatSettings {
                align_separators: false,
                stack_separator: " - ".to_owned(),
                stack_home_as_tilde: true,
                bookmarks_separator: " - ".to_owned(),
                book_home_as_tilde: true,
            },
            styles: StyleSettings {
                warning_style: "default".to_owned(),
                error_style: "default".to_owned(),
                stack_number_style: "default".to_owned(),
                stack_separator_style: "default".to_owned(),
                stack_path_style: "default".to_owned(),
                stack_punct_style: "default".to_owned(),
                bookmarks_name_style: "default".to_owned(),
                bookmarks_seperator_style: "default".to_owned(),
                bookmarks_path_style: "default".to_owned(),
                bookmarks_punct_style: "default".to_owned(),
            },
        };
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
