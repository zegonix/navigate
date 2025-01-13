//! handle the config file and bookmarks stored
//! in said config file

use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Error, Result};
use std::path::PathBuf;
use std::str::FromStr;
use dirs::config_dir;

use super::config::*;
use config_parser::{make_padding_string, apply_format};

#[derive(Debug, Clone)]
pub struct Bookmarks {
    conf_dir: PathBuf,
    bookmarks: HashMap<String, PathBuf>,
}

impl Bookmarks {
    const BOOKMARK_FILE_NAME: &str = "bookmarks.conf";

    /// generates and populates a new instance of Config
    pub fn new() -> Result<Self> {
        let mut bookmarks = Bookmarks {
            conf_dir: PathBuf::new(),
            bookmarks: HashMap::<String, PathBuf>::new(),
        };
        // get home directory path
        bookmarks.conf_dir = match config_dir() {
            Some(value) => value,
            None => return Err(Error::other("-- failed to find configuration directory")),
        };
        // expand home directory path to get configuration directory path
        bookmarks.conf_dir.push("navigate/");
        bookmarks.build_bookmarks()?;

        Ok(bookmarks)
    }

    /// reads and parses the bookmarks file
    fn build_bookmarks(&mut self) -> Result<()> {
        // check if configuration directory exists, if not create it
        if !self.conf_dir.is_dir() {
            fs::create_dir(self.conf_dir.clone())?;
        }

        let mut bookmark_file = self.conf_dir.clone();

        bookmark_file.push(Self::BOOKMARK_FILE_NAME);

        // check if bookmarks file exists, if not create it
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

    /// returns path of bookmark if it exists
    pub fn get_path_by_name(&mut self, name: &str) -> Result<PathBuf> {
        match self.bookmarks.get(name) {
            Some(value) => Ok(value.to_owned()),
            None => Err(Error::other(format!(
                "-- bookmark with name `{}` does not exist",
                name
            ))),
        }
    }

    /// adds a key/value pair to bookmarks and writes the bookmarks file
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

    /// removes a the entry with key=name if it exists, then writes the bookmarks file
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

    /// formats and prints bookmarks to string
    pub fn to_formatted_string(&self, config: &Settings) -> Result<String> {
        let mut buffer = String::new();

        if self.bookmarks.is_empty() {
            buffer.push_str("-- there are no bookmarks defined");
        } else {
            let max_name_len =  match self.bookmarks.keys().map(String::len).max() {
                Some(value) => value,
                None => return Err(Error::other("-- failed to determine maximum bookmark name length")),
            };
            for (mark, path) in &self.bookmarks {
                let padding = make_padding_string(max_name_len - mark.len());
                let name = apply_format(mark, &config.styles.bookmarks_name_style);
                let separator = apply_format(
                    &config.format.bookmarks_separator,
                    &config.styles.bookmarks_seperator_style,
                );
                let path = apply_format(path.to_str().unwrap(), &config.styles.bookmarks_path_style);
                if config.format.align_separators {
                    buffer.push_str(&format!("{}{}{}{}\n", name, padding, separator, path));
                } else {
                    buffer.push_str(&format!("{}{}{}{}\n", name, separator, padding, path));
                }
            }
        }
        Ok(buffer)
    }

    /// writes the bookmarks file
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
