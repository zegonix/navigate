#![allow(dead_code)]

//! handle the config file and bookmarks stored
//! in said config file

use dirs::config_dir;
use std::fs;
use std::{
    io::{
        Error, Result
    },
    path::PathBuf,
};
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
    /// (bool) delete all older occurences of paths in the stack
    #[default_value(false)]
    pub dedup_stack: bool,

    /// (bool) rotate stack to chosen path when jumping to stack entry
    #[default_value(false)]
    pub rotate_stack_on_jump_to_entry: bool,

    /// (bool) show stack when pushing a path to the stack
    #[default_value(false)]
    pub show_stack_on_push: bool,

    /// (bool) show stack when popping a stack entry
    #[default_value(false)]
    pub show_stack_on_pop: bool,

    /// (bool) show invalid stack entries
    #[default_value(false)]
    pub show_invalid_stack_entries: bool,

    /// (bool) show bookmarks when adding, removing or changing to a bookmark
    #[default_value(false)]
    pub show_entries_on_bookmark: bool,

    /// (bool) show invalid bookmarks when displaying bookmarks
    #[default_value(true)]
    pub show_invalid_bookmarks: bool,

    /// (bool) remove invalid bookmarks on call
    #[default_value(false)]
    pub cleanup_bookmarks: bool,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct FormatSettings {
    /// (bool) add padding before the separator if true, after if false
    #[default_value(true)]
    pub align_separators: bool,

    /// (bool) replace home directory path with '~' when displaying the stack or bookmarks
    #[default_value(false)]
    pub show_home_as_tilde: bool,

    /// (bool) hide numbers when displaying the stack
    #[default_value(false)]
    pub stack_hide_numbers: bool,

    /// (string) separator between stack numbers and paths
    #[default_value("' - '")]
    pub stack_separator: String,

    /// (string) separator between bookmark names and paths
    #[default_value("' - '")]
    pub bookmarks_separator: String,
}

#[derive(Debug, Clone, Default, ConfigParser)]
pub struct StyleSettings {
    /// (string) style applied to warnings
    #[style_config]
    #[default_value("'yellow, italic'")]
    pub warning_style: String,

    /// (string) style applied to errors
    #[style_config]
    #[default_value("'red, bold'")]
    pub error_style: String,

    /// (string) style applied to numbers when displaying the stack
    #[style_config]
    #[default_value("'default'")]
    pub stack_number_style: String,

    /// (string) style applied to separators when displaying the stack
    #[style_config]
    #[default_value("'cyan'")]
    pub stack_separator_style: String,

    /// (string) style applied to paths when displaying the stack
    #[style_config]
    #[default_value("'default'")]
    pub stack_path_style: String,

    /// (string) style applied to punctuation (i.e. '/') when displaying the stack
    #[style_config]
    #[default_value("'magenta'")]
    pub stack_punct_style: String,

    /// (string) style applied to punctuation (i.e. '/') when displaying the stack
    #[style_config]
    #[default_value("'default, strikethrough'")]
    pub stack_invalid_style: String,

    /// (string) style applied to bookmark names when displaying the bookmarks
    #[style_config]
    #[default_value("'default'")]
    pub bookmarks_name_style: String,

    /// (string) style applied to separators when displaying the bookmarks
    #[style_config]
    #[default_value("'cyan'")]
    pub bookmarks_seperator_style: String,

    /// (string) style applied to paths when displaying the bookmarks
    #[style_config]
    #[default_value("'default'")]
    pub bookmarks_path_style: String,

    /// (string) style applied to punctuation (i.e. '/') when displaying the bookmarks
    #[style_config]
    #[default_value("'magenta'")]
    pub bookmarks_punct_style: String,

    /// (string) style applied to punctuation (i.e. '/') when displaying the bookmarks
    #[style_config]
    #[default_value("'strikethrough'")]
    pub bookmarks_invalid_style: String,
}

impl Config {
    const CONFIG_DIRECTORY_NAME: &str = "navigate";
    const CONFIG_FILE_NAME: &str = "navigate.toml";
    const DEFAULT_CONFIG_NAME: &str = "default.toml";
    const DEFAULT_FILE_HEADER: &str = "
# default configuration file for `navigate`
#
# value type should be in the comment
# string values have to be quoted, both single and double quotes work
# integer and boolean values do not need quotes
# boolean values are either `true` or `false`
";

    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut config: Config = Self::default();
        // get configuration directory
        let mut config_file: PathBuf = match config_dir() {
            Some(value) => value,
            None => {
                return Err(Error::other("-- failed to retrieve configuration directory"))
            }
        };
        config_file.push(Self::CONFIG_DIRECTORY_NAME);
        if !config_file.is_dir() {
            if fs::create_dir(&config_file).is_err() {
                return Err(Error::other("-- failed to create a directory for the configuration files"));
            }
        }

        // expand path to configuration file and default configuration file
        let mut default_file = config_file.clone();
        default_file.push(Self::DEFAULT_CONFIG_NAME);
        config_file.push(Self::CONFIG_FILE_NAME);

        // write default configuration file if it does not exist
        if !default_file.is_file() {
            let mut default_string = Self::DEFAULT_FILE_HEADER.to_owned();
            default_string.push_str(&config.to_string());
            _ = fs::write(&default_file, default_string);
        }

        // parse configuration file and populate config struct
        // if the file is not found, navigate uses the defaults
        if config_file.is_file() {
            let config_str = match fs::read_to_string(&config_file) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            _ = config.parse_from_string(&config_str);
        } else {
            let default_config = match fs::read_to_string(&default_file) {
                Ok(value) => value,
                Err(error) => return Err(error),
            };
            _ = fs::write(&config_file, &default_config);
            _ = config.parse_from_string(&default_config);
        }

        Ok(config)
    }

    /// formats and prints config to string
    pub fn to_formatted_string(&self) -> Result<String> {
        Ok(format!("{:#?}", self))
    }

}
