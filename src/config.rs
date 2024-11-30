//! handle the config file and bookmarks stored
//! in said config file

use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::{PathBuf};
use std::str::FromStr;
use std::collections::HashMap;
use std::env::var;

#[derive(Debug, Clone)]
pub struct Config {
    pub conf_dir: PathBuf,
    pub bookmarks: HashMap<String, PathBuf>,
}

impl Config {
    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut bookmarks = Config {
            conf_dir: PathBuf::new(),
            bookmarks: HashMap::<String, PathBuf>::new(),
        };
        let home_dir = match var("HOME") {
            Ok(value) => value,
            Err(error) => return Err(Error::other(error.to_string())),
        };
        bookmarks.conf_dir = match PathBuf::from_str(&home_dir) {
            Ok(value) => value,
            Err(error) => return Err(Error::other(error.to_string())),
        };
        bookmarks.conf_dir.push(".config/navigate/");
        bookmarks.build_config()?;

        Ok(bookmarks)
    }

    fn build_config(&mut self) -> Result<()> {
        let mut bookmark_file = self.conf_dir.clone();
        bookmark_file.push("bookmarks.conf");

        if !bookmark_file.is_file() {
            _ = File::create(bookmark_file.clone())?;
        }
        
        let bookmarks = fs::read_to_string(bookmark_file)?;
        let bookmarks = bookmarks.split("\n");
        for entry in bookmarks {
            let tokens: Vec<&str> = entry.split("=").collect();
            if tokens.len() != 2 {
                continue;
            }
            let key: String = String::from(tokens[0]);
            let path = match PathBuf::from_str(tokens[1]) {
                Ok(value) => value,
                Err(err) => return Err(Error::other(err.to_string())),
            };
            if !path.is_dir() {
                continue;
            }
            self.bookmarks.insert(key, path);
        }

        Ok(())
    }
}
