#![allow(dead_code)]

use dirs::config_dir;
use std::collections::HashMap;
use std::fs;
use std::io::{Error, Result};
use std::path::PathBuf;

const DEFAULT_SETTINGS: &[&str] = &[
    "show_stack_on_push=false",
    "show_stack_on_pop=false",
    "show_stack_on_bookmark=false",
    "stack_separator= - ",
    "bookmark_separator= - ",
    "stack_number=default, bold",
    "stack_separator=cyan",
    "stack_path=default",
    "bookmark_name=default",
    "bookmark_separator=cyan",
    "bookmark_path=default",
];

#[derive(Debug, Clone)]
pub struct Config {
    conf_file_path: PathBuf,
    pub settings: HashMap<&'static str, &'static str>,
}

impl Config {
    const CONFIG_DIR_NAME: &str = "navigate/";
    const CONFIG_FILE_NAME: &str = "navigate.conf";

    pub fn new() -> Result<Self> {
        let mut settings = HashMap::<&str, &str>::new();

        // fill the hashmaps with the defined settings and their default values
        for item in DEFAULT_SETTINGS {
            let tokens = item.split(['=']).collect::<Vec<&str>>();
            if tokens.len() != 2 {
                panic!("-- fix default format settings")
            }
            settings.insert(tokens.first().unwrap(), tokens.last().unwrap());
        }

        let mut config = Config {
            conf_file_path: PathBuf::new(),
            settings,
        };

        // get configuration directory
        config.conf_file_path = match config_dir() {
            Some(value) => value,
            None => {
                return Err(Error::other(
                    "-- failed to retrieve configuration directory",
                ))
            }
        };
        // expand path to configuration file
        config.conf_file_path.push(format!(
            "{}{}",
            Self::CONFIG_DIR_NAME,
            Self::CONFIG_FILE_NAME,
        ));

        // parse configuration file and populate config struct
        if config.parse_config().is_err() {
            config.write_config()?;
        }

        // config.parse_color_settings()?;

        Ok(config)
    }

    /// formats and prints config to string
    pub fn to_formatted_string(&self) -> Result<String> {
        // TODO implement
        Ok("hi".to_owned())
    }

    /// parse config file
    fn parse_config(&mut self) -> Result<()> {
        if !self.conf_file_path.is_file() {
            return Err(Error::other("-- config file does not exist"));
        }

        let config = match fs::read_to_string(&self.conf_file_path.clone()) {
            Ok(value) => value,
            Err(error) => return Err(error),
        };

        for line in config.lines() {
            // ignore comments
            if line.starts_with("#") {
                continue;
            }

            let token = line.split(['=']).collect::<Vec<&str>>();
            let key = match token.first() {
                Some(value) => value,
                None => return Err(Error::other(format!("-- failed to parse '{}'", line))),
            };
            let value = match token.last() {
                Some(value) => value,
                None => return Err(Error::other(format!("-- failed to parse '{}'", line))),
            };
            if self.settings.contains_key(key) {
                if let Some(entry) = self.settings.get_mut(key) {
                    *entry = value.clone();
                }
            } else {
                println!("-- ignored unknown setting : {}", line);
            }
        }
        
        // self.settings.entry(key).and_modify(|entry| *entry = value);
        Ok(())
    }

    /// write configuration file
    fn write_config(&self) -> Result<()> {
        let mut buffer = String::new();

        buffer.push_str("# `navigate` settings\n");
        for (name, value) in self.settings.clone() {
            buffer.push_str(&format!("{}={}\n", name, value));
        }
        buffer.push('\n');

        if fs::write(self.conf_file_path.clone(), buffer).is_err() {
            return Err(Error::other("-- failed to write configuration file"));
        }

        Ok(())
    }
}
