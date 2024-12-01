//! handle the config file and bookmarks stored
//! in said config file

use std::collections::HashMap;
use std::env::var;
use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Config {
    conf_dir: PathBuf,
    settings: Settings,
    bookmarks: HashMap<String, PathBuf>,
}

#[derive(Debug, Clone)]
pub struct Settings {}

impl Config {
    const BOOKMARK_FILE_NAME: &str = "bookmarks.conf";

    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut bookmarks = Config {
            conf_dir: PathBuf::new(),
            settings: Settings{},
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

    pub fn get_bookmarks(&mut self) -> &mut HashMap<String, PathBuf> {
        &mut self.bookmarks
    }

    pub fn add_bookmark(&mut self, name: &String, path: &PathBuf) -> Result<()> {
        if !path.is_dir() {
            return Err(Error::other(
                "-- provided path argument does not point to a valid directory",
            ));
        } else {
            self.bookmarks.insert(name.to_string(), path.to_path_buf());
            self.write_bookmark_file()?;
        }
        Ok(())
    }

    pub fn remove_bookmark(&mut self, name: &String) -> Result<()> {
        if self.bookmarks.contains_key(name) {
            _ = self.bookmarks.remove(name);
            self.write_bookmark_file()?;
        } else {
            return Err(Error::other(
                "-- bookmark requested to delete does not exist",
            ));
        }
        Ok(())
    }

    fn build_config(&mut self) -> Result<()> {
        let mut bookmark_file = self.conf_dir.clone();
        bookmark_file.push(Self::BOOKMARK_FILE_NAME);

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

    fn write_bookmark_file(&self) -> Result<()> {
        let mut file_content = String::new();
        for (mark, path) in self.bookmarks.iter() {
            file_content.push_str(&format!("{}={}\n", mark, path.to_str().unwrap()));
        }

        let mut path = self.conf_dir.clone();
        path.push(Self::BOOKMARK_FILE_NAME);

        fs::write(path, file_content)?;
        Ok(())
    }
}
